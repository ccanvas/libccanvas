use std::{collections::BTreeSet, error::Error, io};

use crate::{
    client::Client, clientconfig::ClientConfig, defaults, pass::{BytePass, OnDropPass, PacketReqPass, PacketResPass}
};

pub struct ClientBuilder {
    config: ClientConfig,
    recieved_bytes: BTreeSet<(Option<usize>, BytePass)>,
    packet_req_pass: BTreeSet<(Option<usize>, PacketReqPass)>,
    packet_res_pass: BTreeSet<(Option<usize>, PacketResPass)>,
    drop_pass: BTreeSet<(Option<usize>, OnDropPass)>
}

impl ClientBuilder {
    pub fn new(config: ClientConfig) -> Self {
        Self {
            config,
            recieved_bytes: BTreeSet::new(),
            packet_req_pass: BTreeSet::new(),
            packet_res_pass: BTreeSet::new(),
            drop_pass: BTreeSet::new()
        }
    }

    pub fn add_core(self) -> Self {
        self.on_drop(None, defaults::on_drop)
    }

    pub fn on_drop(mut self, priority: Option<usize>, map: OnDropPass) -> Self {
        self.drop_pass.insert((priority, map));
        self
    }

    pub fn on_byte_recieve(
        mut self,
        priority: Option<usize>,
        map: BytePass,
    ) -> Self {
        self.recieved_bytes.insert((priority, map));
        self
    }

    pub fn build(self) -> Result<Client, io::Error> {
        self.try_into()
    }

    pub fn get_byte_recieve(&self) -> Vec<BytePass> {
        self.recieved_bytes.iter().map(|item| item.1).collect()
    }

    pub fn get_drop(&self) -> Vec<OnDropPass> {
        self.drop_pass.iter().map(|item| item.1).collect()
    }

    pub fn get_config(&self) -> &ClientConfig {
        &self.config
    }
}
