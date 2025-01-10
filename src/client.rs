use std::{collections::HashMap, fs, io, path::PathBuf};

use mio::{
    net::{UnixListener, UnixStream},
    Token,
};

use crate::{clientbuilder::ClientBuilder, pass::{BytePass, OnDropPass}};

pub struct Client {
    listener: UnixListener,
    listener_path: PathBuf,
    incoming_streams: HashMap<Token, UnixStream>,
    byte_recieve: Vec<BytePass>,
    drop: Vec<OnDropPass>
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
            byte_recieve: builder.get_byte_recieve(),
            drop: builder.get_drop()
        })
    }

    type Error = io::Error;
}

impl Drop for Client {
    fn drop(&mut self) {
        for pass in self.drop.iter() {
            pass(&self)
        }
    }
}
