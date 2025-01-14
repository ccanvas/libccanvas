use std::{io, path::PathBuf};

use mio::net::UnixListener;

pub struct ClientConfig {
    listening: PathBuf,
    events_capacity: usize,
}

impl ClientConfig {
    pub fn new(listening: PathBuf) -> Self {
        Self {
            listening,
            events_capacity: 1024,
        }
    }

    pub fn with_capacity(listening: PathBuf, events_capacity: usize) -> Self {
        Self {
            listening,
            events_capacity,
        }
    }

    pub fn listening(&self) -> &PathBuf {
        &self.listening
    }

    pub fn get_capacity(&self) -> usize {
        self.events_capacity
    }

    pub fn connect(&self) -> Result<UnixListener, io::Error> {
        UnixListener::bind(&self.listening)
    }
}
