use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Dimension {
    pub width: u32,
    pub height: u32,
}

impl Dimension {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}
