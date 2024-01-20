use serde::Serialize;

use crate::bindings::{Discriminator, KeyCode, KeyEvent, KeyModifier, MouseType};

#[derive(Hash, PartialEq, Eq, Serialize, Debug, Clone)]
#[serde(tag = "type")]
/// A single subscription channel.
pub enum Subscription {
    /// Every single event
    #[serde(rename = "everything")]
    Everything,
    /// All key press events
    #[serde(rename = "all key presses")]
    AllKeyPresses,
    /// All mouse click and drag events
    #[serde(rename = "all mouse events")]
    AllMouseEvents,
    /// All messages from other components
    #[serde(rename = "all messages")]
    AllMessages,
    /// A specific key event
    #[serde(rename = "specific key press")]
    SpecificKeyPress { key: KeyEvent },
    /// All key events with that key modifier
    #[serde(rename = "specific key modifier")]
    SpecificKeyModifier { modifier: KeyModifier },
    /// All key events with that key code
    #[serde(rename = "specific key code")]
    SpecificKeyCode { code: KeyCode },
    /// A specific mouse event
    #[serde(rename = "specific mouse event")]
    SpecificMouseEvent { mouse: MouseType },
    /// A message from a specific component
    #[serde(rename = "specific message")]
    SpecificMessage { source: Discriminator },
    /// Screen resize events
    #[serde(rename = "screen resize")]
    ScreenResize,
    #[serde(rename = "focused")]
    /// Current space focused
    Focused,
    #[serde(rename = "unfocused")]
    /// Current space unfocused
    Unfocused,

    #[serde(rename = "multiple")]
    /// Subscribe to multiple channels at once
    Multiple {
        subs: Vec<(Subscription, Option<u32>)>,
    },
}

impl Subscription {
    pub fn specific_keypress(key: KeyEvent) -> Self {
        Self::SpecificKeyPress { key }
    }

    pub fn specific_keycode(code: KeyCode) -> Self {
        Self::SpecificKeyCode { code }
    }

    pub fn specific_keymodifier(modifier: KeyModifier) -> Self {
        Self::SpecificKeyModifier { modifier }
    }

    pub fn specific_mouse(mousetype: MouseType) -> Self {
        Self::SpecificMouseEvent { mouse: mousetype }
    }

    pub fn specific_message(source: Discriminator) -> Self {
        Self::SpecificMessage { source }
    }

    pub fn with_priority(self, priority: u32) -> (Self, Option<u32>) {
        (self, Some(priority))
    }
}

impl From<Subscription> for (Subscription, Option<u32>) {
    fn from(value: Subscription) -> Self {
        (value, None)
    }
}
