use std::fs;

use crate::Client;

pub fn on_drop(client: &mut Client) {
    let _ = fs::remove_file(client.get_listener_path());
}
