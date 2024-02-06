use serde::{Deserialize, Serialize};

use super::Dimension;

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl Rect {
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

impl From<Dimension> for Rect {
    fn from(value: Dimension) -> Self {
        Self {
            x: 0,
            y: 0,
            width: value.width,
            height: value.height,
        }
    }
}
