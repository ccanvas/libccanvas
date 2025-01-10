use std::{io, path::PathBuf};

use mio::net::{UnixListener, UnixStream};

pub struct ClientConfig {
    listening: PathBuf,
    ccanvas: PathBuf,
}

impl ClientConfig {
    pub fn new(listening: PathBuf, ccanvas: PathBuf) -> Self {
        Self { listening, ccanvas }
    }

    pub fn listening(&self) -> &PathBuf {
        &self.listening
    }

    pub fn connect(&self) -> Result<(UnixListener, UnixStream), io::Error> {
        Ok((
            UnixListener::bind(&self.listening)?,
            UnixStream::connect(&self.ccanvas)?,
        ))
    }
}
