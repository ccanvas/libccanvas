use std::path::PathBuf;

pub struct ClientConfig {
    /// Path to the listener socket
    pub listener_socket: PathBuf,
    /// Path to request socket
    pub request_socket: PathBuf,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            listener_socket: PathBuf::from("listen.sock"),
            request_socket: PathBuf::from("requests.sock"),
        }
    }
}
