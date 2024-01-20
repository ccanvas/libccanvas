use tokio::sync::mpsc::UnboundedSender;

use crate::bindings::{Discriminator, Request, RequestContent, Subscription};

/// Suppress a channel throughout the lifetime of the struct
pub struct LifetimeSuppressor {
    id: u32,
    channel: Subscription,
    target: Discriminator,
    sender: Option<UnboundedSender<Request>>,
}

impl LifetimeSuppressor {
    pub fn new(
        id: u32,
        channel: Subscription,
        target: Discriminator,
        sender: UnboundedSender<Request>,
    ) -> Self {
        Self {
            id,
            channel,
            target,
            sender: Some(sender),
        }
    }

    /// Deconstruct self into inner elements.
    pub fn deconstruct(
        &self,
    ) -> Option<(u32, Subscription, Discriminator, UnboundedSender<Request>)> {
        self.sender.as_ref().map(|_| {
            (
                self.id,
                self.channel.clone(),
                self.target.clone(),
                self.sender.as_ref().unwrap().clone(),
            )
        })
    }
}

impl Drop for LifetimeSuppressor {
    fn drop(&mut self) {
        if self.sender.is_none() {
            return;
        }

        let req = Request::new(
            self.target.clone(),
            RequestContent::Unsuppress {
                channel: self.channel.clone(),
                id: self.id,
            },
        );
        self.sender.as_ref().unwrap().send(req).unwrap();
    }
}
