use crate::{bindings::Discriminator, client::Client};

use super::ScrollComponent;

impl Client {
    /// Returns a scroll component from a discrim of ccanvas-scroll
    pub fn scroll(&self, discrim: Discriminator) -> ScrollComponent {
        ScrollComponent::new(discrim, self.clone())
    }
}
