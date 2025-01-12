use ccanvas_bindings::packets::Packet;

use crate::client::Client;

pub type PacketPass = fn(Packet) -> Option<Packet>;
pub type ClientPass = fn(&Client);
