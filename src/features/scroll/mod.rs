mod request;
use std::{
    collections::HashMap,
    sync::{Mutex, OnceLock},
};

pub use request::*;
mod client;
mod scrollcomponent;
pub use scrollcomponent::*;
use tokio::sync::oneshot;

pub const SCROLL_REQ: &str = "!scroll-request";
pub const SCROLL_RES: &str = "!scroll-response";
pub const SCROLL_READY: &str = "!scroll-ready";

pub static SCROLL_CONFIRMS: OnceLock<Mutex<HashMap<u32, oneshot::Sender<ScrollResponseVariant>>>> =
    OnceLock::new();
