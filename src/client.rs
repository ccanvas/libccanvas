use std::{
    cmp,
    collections::{HashMap, VecDeque},
    io::{self, BorrowedBuf, Read, Write},
    path::{Path, PathBuf},
    time::Duration,
};

use ccanvas_bindings::packets::{connection, Packet};
use mio::{
    event::Event,
    net::{UnixListener, UnixStream},
    Events, Interest, Poll, Token,
};

use crate::{
    clientbuilder::ClientBuilder,
    pass::{ClientMutPass, PacketParser, PacketPass},
};

pub struct Client {
    listener: UnixListener,
    listener_path: PathBuf,

    awaiting_outbound_connections: HashMap<Token, UnixStream>,
    outbound_connections: HashMap<Token, UnixStream>,
    inbound_connections: HashMap<Token, UnixStream>,
    inbound_remap: HashMap<Token, Token>,
    poll: Poll,
    events: Events,
    events_buf: VecDeque<(Token, Packet)>,

    req_pass: Vec<PacketPass>,
    recv_pass: Vec<PacketPass>,
    drop_pass: Vec<ClientMutPass>,

    parser: PacketParser,

    connection_token: Token,
}

impl Client {
    pub fn get_listener_path(&self) -> &PathBuf {
        &self.listener_path
    }
}

impl Client {
    // poll one
    pub fn poll(&mut self) -> Result<(Token, Packet), io::Error> {
        let res = self.poll.poll(&mut self.events, Some(Duration::ZERO));
        if let Err(err) = &res {
            if err.kind() != io::ErrorKind::Interrupted {
                res.unwrap()
            }
        }

        loop {
            let mut new_events = Vec::new();
            let mut new_conn: usize = 0;

            for event in self.events.iter() {
                match event.token() {
                    // i dont know why a loop is needed here, or if its needed
                    // gotta figure this out later
                    Token(0) => loop {
                        let (mut connection, _) = match self.listener.accept() {
                            Ok(res) => res,
                            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                                break;
                            }
                            e => {
                                e.unwrap();
                                unreachable!()
                            }
                        };

                        new_conn += 1;
                        let token = Token(self.connection_token.0 + new_conn);

                        self.poll
                            .registry()
                            .register(&mut connection, token, Interest::READABLE)
                            .unwrap();
                        self.inbound_connections.insert(token, connection);
                    },
                    token => new_events.push((event, token)),
                }
            }

            self.connection_token.0 += new_conn;

            for (event, token) in new_events.into_iter() {
                let connection = match self.inbound_connections.get_mut(&token) {
                    Some(conn) => conn,
                    None => {
                        if let Some(remapped) = self.inbound_remap.get(&token) {
                            match self.inbound_connections.get_mut(remapped) {
                                Some(conn) => conn,
                                None => continue,
                            }
                        } else {
                            continue;
                        }
                    }
                };

                if let Some(bytes) = Self::handle_event(connection, event) {
                    if let Some(packet) = (self.parser)(&bytes) {
                        self.events_buf.push_back((token, packet));
                    }
                }
            }

            'dequeue: while let Some((mut token, mut packet)) = self.events_buf.pop_front() {
                for i in 0..self.recv_pass.len() {
                    match self.recv_pass[i](self, packet, &mut token) {
                        Some(mapped) => packet = mapped,
                        None => continue 'dequeue,
                    }
                }

                return Ok((token, packet));
            }

            self.inbound_remap.clear();

            let res = self.poll.poll(&mut self.events, None);
            if let Err(err) = &res {
                if err.kind() != io::ErrorKind::Interrupted {
                    res.unwrap()
                }
            }
        }
    }

    fn handle_event(connection: &mut UnixStream, event: &Event) -> Option<Vec<u8>> {
        if event.is_readable() {
            let mut recieved_data = Vec::new();

            Self::default_read_to_end(connection, &mut recieved_data, None);

            if recieved_data.is_empty() {
                None
            } else {
                Some(recieved_data)
            }
        } else {
            None
        }
    }
}

impl Client {
    pub fn message(&mut self, mut token: Token, mut packet: Packet) -> io::Result<()> {
        for i in 0..self.req_pass.len() {
            match self.req_pass[i](self, packet, &mut token) {
                Some(mapped) => packet = mapped,
                None => return Ok(()),
            }
        }

        match self.outbound_connections.get_mut(&token) {
            Some(conn) => Client::write(conn, &packet.to_bytes()),
            None => Err(io::Error::new(
                io::ErrorKind::NotFound,
                "connection not found",
            )),
        }
    }

    pub fn message_bytes(&mut self, token: &Token, bytes: &[u8]) -> io::Result<()> {
        match self.outbound_connections.get_mut(token) {
            Some(conn) => Client::write(conn, bytes),
            None => Err(io::Error::new(
                io::ErrorKind::NotFound,
                "connection not found",
            )),
        }
    }

    fn write(stream: &mut UnixStream, bytes: &[u8]) -> io::Result<()> {
        stream.write_all(bytes)
    }

    // reimplementation of `read_to_end` that returns the read content
    // when WouldBlock is encountered
    fn default_read_to_end<R: Read + ?Sized>(
        r: &mut R,
        buf: &mut Vec<u8>,
        size_hint: Option<usize>,
    ) {
        let start_cap = buf.capacity();
        // Optionally limit the maximum bytes read on each iteration.
        // This adds an arbitrary fiddle factor to allow for more data than we expect.
        let max_read_size =
            size_hint.and_then(|s| s.checked_add(1024)?.checked_next_power_of_two());

        let mut initialized = 0; // Extra initialized bytes from previous loop iteration
        loop {
            if buf.len() == buf.capacity() {
                buf.reserve(32); // buf is full, need more space
            }

            let mut spare = buf.spare_capacity_mut();
            if let Some(size) = max_read_size {
                let len = cmp::min(spare.len(), size);
                spare = &mut spare[..len]
            }

            let mut read_buf: BorrowedBuf<'_> = spare.into();

            // SAFETY: These bytes were initialized but not filled in the previous loop
            unsafe {
                read_buf.set_init(initialized);
            }

            let mut cursor = read_buf.unfilled();
            match r.read_buf(cursor.reborrow()) {
                Ok(()) => {}
                Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {}
                Err(_) => return,
            }

            if cursor.written() == 0 {
                return;
            }

            // store how much was initialized but not filled
            initialized = cursor.init_ref().len();

            // SAFETY: BorrowedBuf's invariants mean this much memory is initialized.
            unsafe {
                let new_len = read_buf.filled().len() + buf.len();
                buf.set_len(new_len);
            }

            if buf.len() == buf.capacity() && buf.capacity() == start_cap {
                // The buffer might be an exact fit. Let's read into a probe buffer
                // and see if it returns `Ok(0)`. If so, we've avoided an
                // unnecessary doubling of the capacity. But if not, append the
                // probe buffer to the primary buffer and let its capacity grow.
                let mut probe = [0u8; 32];

                loop {
                    match r.read(&mut probe) {
                        Ok(0) => return,
                        Ok(n) => {
                            buf.extend_from_slice(&probe[..n]);
                            break;
                        }
                        Err(ref e) if e.kind() == io::ErrorKind::Interrupted => continue,
                        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => break,
                        Err(_) => return,
                    }
                }
            }
        }
    }
}

impl Client {
    pub fn connect(&mut self, self_ident: Vec<u8>, path: &Path) -> io::Result<()> {
        self.connection_token.0 += 1;

        let mut stream = UnixStream::connect(path)?;
        let packet = Packet::from(connection::Group::ReqConn {
            label: self_ident,
            socket: Some((
                self.listener_path.to_string_lossy().to_string(),
                self.connection_token.0.to_le_bytes().to_vec(),
            )),
        })
        .to_bytes();

        Self::write(&mut stream, &packet)?;

        self.awaiting_outbound_connections
            .insert(self.connection_token, stream);

        Ok(())
    }

    pub fn approve_connection(&mut self, awaiting_outbound: Token, inbound: Token) -> bool {
        if self
            .awaiting_outbound_connections
            .contains_key(&awaiting_outbound)
        {
            if let Some(mut await_in) = self.inbound_connections.remove(&inbound) {
                self.poll
                    .registry()
                    .reregister(&mut await_in, awaiting_outbound, Interest::READABLE)
                    .unwrap();

                self.outbound_connections.insert(
                    awaiting_outbound,
                    self.awaiting_outbound_connections
                        .remove(&awaiting_outbound)
                        .unwrap(),
                );
                self.inbound_connections.insert(awaiting_outbound, await_in);
                return true;
            }
        }

        false
    }

    pub fn disapprove_connection(&mut self, awaiting_outbound: Token, inbound: Token) -> bool {
        self.awaiting_outbound_connections
            .remove(&awaiting_outbound)
            .is_some()
            && self.inbound_connections.remove(&inbound).is_some()
    }
}

impl TryFrom<ClientBuilder> for Client {
    fn try_from(builder: ClientBuilder) -> Result<Self, Self::Error> {
        let mut listener = builder.get_config().connect()?;
        let poll = Poll::new().unwrap();
        poll.registry()
            .register(&mut listener, Token(0), Interest::READABLE)
            .unwrap();

        Ok(Self {
            listener,
            listener_path: builder.get_config().listening().clone(),

            awaiting_outbound_connections: HashMap::new(),
            inbound_connections: HashMap::new(),
            outbound_connections: HashMap::new(),
            inbound_remap: HashMap::new(),
            poll,
            events: Events::with_capacity(builder.get_config().get_capacity()),
            events_buf: VecDeque::new(),

            req_pass: builder.get_req(),
            recv_pass: builder.get_recv(),
            drop_pass: builder.get_drop(),

            parser: builder.get_parser(),

            connection_token: Token(0),
        })
    }

    type Error = io::Error;
}

impl Drop for Client {
    fn drop(&mut self) {
        for i in 0..self.drop_pass.len() {
            self.drop_pass[i](self)
        }
    }
}
