use std::{collections::HashMap, io, path::PathBuf};

use mio::{
    net::{UnixListener, UnixStream},
    Token,
};

use crate::{
    clientbuilder::ClientBuilder,
    pass::{ClientPass, PacketPass},
};

pub struct Client {
    listener: UnixListener,
    listener_path: PathBuf,
    incoming_streams: HashMap<Token, UnixStream>,
    req_pass: Vec<PacketPass>,
    recv_pass: Vec<PacketPass>,
    drop_pass: Vec<ClientPass>,
}

impl Client {
    pub fn get_listener_path(&self) -> &PathBuf {
        &self.listener_path
    }
}

impl TryFrom<ClientBuilder> for Client {
    fn try_from(builder: ClientBuilder) -> Result<Self, Self::Error> {
        let (listener, ccanvas) = builder.get_config().connect()?;

        Ok(Self {
            listener,
            listener_path: builder.get_config().listening().clone(),
            incoming_streams: HashMap::from([(Token(0), ccanvas)]),
            req_pass: builder.get_req(),
            recv_pass: builder.get_recv(),
            drop_pass: builder.get_drop(),
        })
    }

    type Error = io::Error;
}

impl Drop for Client {
    fn drop(&mut self) {
        for pass in self.drop_pass.iter() {
            pass(self)
        }
    }
}
