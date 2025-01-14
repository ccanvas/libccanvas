use ccanvas_bindings::packets::Packet;
use mio::Token;

use crate::client::Client;

pub type PacketPass = fn(&mut Client, Packet, &mut Token) -> Option<Packet>;
pub type ClientMutPass = fn(&mut Client);
pub type PacketParser = fn(&[u8]) -> Option<Packet>;
