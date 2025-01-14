use std::{collections::BTreeSet, io};

use ccanvas_bindings::packets::Packet;

use crate::{
    client::Client,
    clientconfig::ClientConfig,
    defaults,
    pass::{ClientMutPass, PacketParser, PacketPass},
};

pub struct ClientBuilder {
    config: ClientConfig,
    req_pass: BTreeSet<(Option<usize>, PacketPass)>,
    recv_pass: BTreeSet<(Option<usize>, PacketPass)>,
    drop_pass: BTreeSet<(Option<usize>, ClientMutPass)>,
    parser: PacketParser,
}

impl ClientBuilder {
    pub fn new(config: ClientConfig) -> Self {
        Self {
            config,
            req_pass: BTreeSet::new(),
            recv_pass: BTreeSet::new(),
            drop_pass: BTreeSet::new(),
            parser: Packet::from_bytes,
        }
    }

    pub fn add_core(self) -> Self {
        self.on_recv(Some(1), defaults::on_recv)
            .on_drop(None, defaults::on_drop)
    }

    pub fn on_send(mut self, priority: Option<usize>, map: PacketPass) -> Self {
        self.req_pass.insert((priority, map));
        self
    }

    pub fn on_recv(mut self, priority: Option<usize>, map: PacketPass) -> Self {
        self.recv_pass.insert((priority, map));
        self
    }

    pub fn on_drop(mut self, priority: Option<usize>, map: ClientMutPass) -> Self {
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

    pub fn get_drop(&self) -> Vec<ClientMutPass> {
        self.drop_pass.iter().map(|item| item.1).collect()
    }

    pub fn get_config(&self) -> &ClientConfig {
        &self.config
    }

    pub fn get_parser(&self) -> PacketParser {
        self.parser
    }
}
