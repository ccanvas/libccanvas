use serde::{Deserialize, Serialize};

/// A unique identifier for every component.
#[derive(Default, PartialEq, Eq, Clone, Debug, Deserialize, Serialize, Hash)]
pub struct Discriminator(Vec<u32>);

impl Discriminator {
    /// Creates a new discriminator from `Vec<u32>`
    pub fn new(path: Vec<u32>) -> Self {
        Self(path)
    }

    /// Returns the discriminator of the master space `[1]`
    pub fn master() -> Self {
        Self(vec![1])
    }
}
