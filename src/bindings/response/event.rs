use tokio::sync::mpsc::UnboundedSender;

use crate::bindings::{Discriminator, Request, RequestContent};

use super::EventVariant;

/// An event binding struct that sends a confirmation to the ccanvas server.
pub struct Event {
    /// real content of the event
    content: EventVariant,
    /// confirmation handle to release the event
    confirm: Option<(u32, UnboundedSender<Request>)>,
}

impl Event {
    pub fn new(content: EventVariant, sender: UnboundedSender<Request>, confirm: u32) -> Self {
        Self {
            content,
            confirm: Some((confirm, sender)),
        }
    }

    /// Marks the event as done and releases the event.
    ///
    /// - `true` allows other components to also recieve the event.
    /// - `false` captures the event.
    pub fn done(&mut self, pass: bool) {
        if let Some((id, sender)) = std::mem::take(&mut self.confirm) {
            sender
                .send(Request::new(
                    Discriminator::default(),
                    RequestContent::ConfirmRecieve { id, pass },
                ))
                .unwrap();
        }
    }

    /// Returns a reference to the content of the event.
    pub fn get(&self) -> &EventVariant {
        &self.content
    }

    /// Returns a mutable reference to the content of the event.
    pub fn get_mut(&mut self) -> &mut EventVariant {
        &mut self.content
    }
}

/// Alias to `event.done(true)`.
impl Drop for Event {
    fn drop(&mut self) {
        self.done(true)
    }
}
