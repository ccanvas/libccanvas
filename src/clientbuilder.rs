use std::{collections::BTreeSet, io};

use crate::{
    client::Client,
    clientconfig::ClientConfig,
    defaults,
    pass::{ClientPass, PacketPass},
};

pub struct ClientBuilder {
    config: ClientConfig,
    req_pass: BTreeSet<(Option<usize>, PacketPass)>,
    recv_pass: BTreeSet<(Option<usize>, PacketPass)>,
    drop_pass: BTreeSet<(Option<usize>, ClientPass)>,
}

impl ClientBuilder {
    pub fn new(config: ClientConfig) -> Self {
        Self {
            config,
            req_pass: BTreeSet::new(),
            recv_pass: BTreeSet::new(),
            drop_pass: BTreeSet::new(),
        }
    }

    pub fn add_core(self) -> Self {
        self.on_drop(None, defaults::on_drop)
    }

    pub fn on_req(mut self, priority: Option<usize>, map: PacketPass) -> Self {
        self.req_pass.insert((priority, map));
        self
    }

    pub fn on_recv(mut self, priority: Option<usize>, map: PacketPass) -> Self {
        self.recv_pass.insert((priority, map));
        self
    }

    pub fn on_drop(mut self, priority: Option<usize>, map: ClientPass) -> Self {
        self.drop_pass.insert((priority, map));
        self
    }

    pub fn build(self) -> Result<Client, io::Error> {
        self.try_into()
    }

    pub fn get_req(&self) -> Vec<PacketPass> {
        self.req_pass.iter().map(|item| item.1).collect()
    }

    pub fn get_recv(&self) -> Vec<PacketPass> {
        self.recv_pass.iter().map(|item| item.1).collect()
    }

    pub fn get_drop(&self) -> Vec<ClientPass> {
        self.drop_pass.iter().map(|item| item.1).collect()
    }

    pub fn get_config(&self) -> &ClientConfig {
        &self.config
    }
}
