use ccanvas_bindings::packets::{PacketReq, PacketRes};

use crate::client::Client;

pub type BytePass = fn(Vec<u8>) -> Option<Vec<u8>>;
pub type PacketResPass = fn(PacketRes) -> Option<PacketRes>;
pub type PacketReqPass = fn(PacketReq) -> Option<PacketReq>;
pub type OnDropPass = fn(&Client);
