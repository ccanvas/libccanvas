//! basically copied from the main repo of /ccanvas
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::bindings::Discriminator;

/// All possible events you can get from `client.recv()`.
#[derive(Deserialize, Clone, PartialEq, Debug)]
#[serde(tag = "type")]
pub enum EventVariant {
    /// A keyboard event
    #[serde(rename = "key")]
    Key(KeyEvent),
    /// A mouse event
    #[serde(rename = "mouse")]
    Mouse(MouseEvent),
    /// A screen resize event
    #[serde(rename = "resize")]
    Resize { width: u32, height: u32 },
    /// A message recieve
    #[serde(rename = "message")]
    Message {
        sender: Discriminator,
        target: Discriminator,
        content: String,
    },
    /// Parent space focused
    #[serde(rename = "focused")]
    Focused,
    /// Parent space unfocused
    #[serde(rename = "unfocused")]
    Unfocused,
    /// Value of a variable has been changed
    #[serde(rename = "value updated")]
    ValueUpdated {
        label: String,
        new: Value,
        discrim: Discriminator,
    },
    /// A variable has been removed
    #[serde(rename = "value removed")]
    ValueRemoved {
        label: String,
        discrim: Discriminator,
    },
}

/// A keyboard event
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Hash)]
pub struct KeyEvent {
    /// The keycode represented by the key pressed.
    pub code: KeyCode,
    /// Key modifiers (e.g. ctrl).
    ///
    /// Note that key modifiers are modifiers. They cannot exist on their own.
    pub modifier: KeyModifier,
}

impl KeyEvent {
    /// Create a new KeyEvent from KeyCode and KeyModifier
    pub fn new(code: KeyCode, modifier: KeyModifier) -> Self {
        Self { code, modifier }
    }
}

/// A single key modifier
///
/// Shift is not a modifier, to see if a key press is "modified" by shift,
/// check for the effect of shift, e.g. upper case alphabets.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Hash)]
pub enum KeyModifier {
    #[serde(rename = "alt")]
    Alt,
    /// Certain keys may not be modified with ctrl, due to limitations of terminals.
    #[serde(rename = "ctrl")]
    Ctrl,
    #[serde(rename = "none")]
    None,
}

/// A single key code
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Hash)]
pub enum KeyCode {
    /// Backspace.
    #[serde(rename = "backspace")]
    Backspace,
    /// Left arrow.
    #[serde(rename = "left")]
    Left,
    /// Right arrow.
    #[serde(rename = "right")]
    Right,
    /// Up arrow.
    #[serde(rename = "up")]
    Up,
    /// Down arrow.
    #[serde(rename = "down")]
    Down,
    /// Home key.
    #[serde(rename = "home")]
    Home,
    /// End key.
    #[serde(rename = "end")]
    End,
    /// Page Up key.
    #[serde(rename = "pageup")]
    PageUp,
    /// Page Down key.
    #[serde(rename = "pagedown")]
    PageDown,
    /// Backward Tab key.
    #[serde(rename = "backtab")]
    BackTab,
    /// Delete key.
    #[serde(rename = "delete")]
    Delete,
    /// Insert key.
    #[serde(rename = "insert")]
    Insert,
    /// Function keys.
    ///
    /// Only function keys 1 through 12 are supported.
    #[serde(rename = "f")]
    F(u8),
    /// Normal character.
    #[serde(rename = "char")]
    Char(char),
    /// Null byte.
    #[serde(rename = "null")]
    Null,
    /// Esc key.
    #[serde(rename = "esc")]
    Esc,
}

/// A single mouse event.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct MouseEvent {
    pub x: u32,
    pub y: u32,
    pub mousetype: MouseType,
}

/// What kind of event is the mouse event.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum MouseType {
    #[serde(rename = "left")]
    /// The left mouse button.
    Left,
    #[serde(rename = "right")]
    /// The right mouse button.
    Right,
    #[serde(rename = "middle")]
    /// The middle mouse button.
    Middle,
    #[serde(rename = "wheelup")]
    /// Mouse wheel is going up.
    ///
    /// This event is typically only used with Mouse::Press.
    WheelUp,
    #[serde(rename = "wheeldown")]
    /// Mouse wheel is going down.
    ///
    /// This event is typically only used with Mouse::Press.
    WheelDown,
    #[serde(rename = "release")]
    /// mouse release
    Release,
    #[serde(rename = "hold")]
    /// is only emitted when u move the mouse, and only applies to left click
    Hold,
}
