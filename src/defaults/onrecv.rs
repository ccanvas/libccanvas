use ccanvas_bindings::packets::{connection, Packet};
use mio::Token;

use crate::Client;

pub fn on_recv(client: &mut Client, packet: Packet, token: &mut Token) -> Option<Packet> {
    match &packet {
        Packet::Connection(connection::Group::ApprConn { echo }) => {
            let new_token = Token(le_usize(echo));
            if client.approve_connection(new_token, *token) {
                *token = new_token;
            }
        }
        Packet::Connection(connection::Group::RejConn { echo }) => {
            client.disapprove_connection(Token(le_usize(echo)), *token);
        }
        _ => {}
    }

    Some(packet)
}

const USIZE_BYTES: usize = std::mem::size_of::<usize>();

fn le_usize(bytes: &[u8]) -> usize {
    let mut arr: [u8; USIZE_BYTES] = unsafe {
        #[allow(invalid_value)]
        #[allow(clippy::uninit_assumed_init)]
        std::mem::MaybeUninit::uninit().assume_init()
    };

    arr[..USIZE_BYTES].copy_from_slice(bytes);

    usize::from_le_bytes(arr)
}
