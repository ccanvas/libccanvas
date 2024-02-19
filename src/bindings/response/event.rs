use std::sync::Mutex;

use tokio::sync::mpsc::UnboundedSender;

use crate::bindings::{Discriminator, Request, RequestContent};

use super::EventVariant;

/// An event binding struct that sends a confirmation to the ccanvas server.
pub struct Event {
    /// real content of the event
    content: EventVariant,
    /// confirmation handle to release the event
    confirm: Mutex<Option<(u32, UnboundedSender<Request>)>>,

    #[cfg(feature = "layout")]
    // sender, this discrim
    // confirm to layout that it is rendered
    layout_confirm: Mutex<Option<(UnboundedSender<Request>, Discriminator)>>,
}

impl Event {
    pub fn new(
        content: EventVariant,
        sender: UnboundedSender<Request>,
        confirm: u32,
        #[cfg(feature = "layout")] self_discrim: Discriminator,
        #[cfg(feature = "layout")] is_layout: bool,
    ) -> Self {
        Self {
            content,
            confirm: Mutex::new(Some((confirm, sender.clone()))),
            #[cfg(feature = "layout")]
            layout_confirm: Mutex::new(if is_layout {
                Some((sender, self_discrim))
            } else {
                None
            }),
        }
    }

    /// Marks the event as done and releases the event.
    ///
    /// - `true` allows other components to also recieve the event.
    /// - `false` captures the event.
    pub fn done(&self, pass: bool) {
        if let Some((id, sender)) = std::mem::take(&mut *self.confirm.lock().unwrap()) {
            sender
                .send(Request::new(
                    Discriminator::default(),
                    RequestContent::ConfirmRecieve { id, pass },
                ))
                .unwrap();
        }

        #[cfg(feature = "layout")]
        {
            let mut layout_confirm = self.layout_confirm.lock().unwrap();
            if let Some((sender, discrim)) = std::mem::take(&mut *layout_confirm) {
                sender
                    .send(Request::new(
                        discrim,
                        RequestContent::SetEntry {
                            label: crate::features::layout::LAYOUT_CONFIRM.to_string(),
                            value: serde_json::Value::Null,
                        },
                    ))
                    .unwrap();
            }
        }
    }

    /// Returns a reference to the content of the event.
    pub fn get(&self) -> &EventVariant {
        &self.content
    }
}

/// Alias to `event.done(true)`.
impl Drop for Event {
    fn drop(&mut self) {
        self.done(true)
    }
}
