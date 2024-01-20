//! Improved serde structs.

use serde::{Deserialize, Serialize};

use crate::bindings::{KeyCode, KeyEvent, KeyModifier};

/// A keyboard event
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Hash)]
pub struct KeyEventBetterSerde {
    /// The keycode represented by the key pressed.
    #[serde(flatten)]
    pub code: KeyCodeBetterSerde,
    /// Key modifiers (e.g. ctrl).
    ///
    /// Note that key modifiers are modifiers. They cannot exist on their own.
    pub modifier: KeyModifier,
}

/// A single key code
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Hash)]
#[serde(tag = "type")]
pub enum KeyCodeBetterSerde {
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
    F { value: u8 },
    /// Normal character.
    #[serde(rename = "char")]
    Char { value: char },
    /// Null byte.
    #[serde(rename = "null")]
    Null,
    /// Esc key.
    #[serde(rename = "esc")]
    Esc,
}

impl From<KeyEventBetterSerde> for KeyEvent {
    fn from(value: KeyEventBetterSerde) -> Self {
        Self {
            code: value.code.into(),
            modifier: value.modifier,
        }
    }
}

impl From<KeyCodeBetterSerde> for KeyCode {
    fn from(value: KeyCodeBetterSerde) -> Self {
        match value {
            KeyCodeBetterSerde::Backspace => Self::Backspace,
            KeyCodeBetterSerde::Left => Self::Left,
            KeyCodeBetterSerde::Right => Self::Right,
            KeyCodeBetterSerde::Up => Self::Up,
            KeyCodeBetterSerde::Down => Self::Down,
            KeyCodeBetterSerde::Home => Self::Home,
            KeyCodeBetterSerde::End => Self::End,
            KeyCodeBetterSerde::PageUp => Self::PageUp,
            KeyCodeBetterSerde::PageDown => Self::PageDown,
            KeyCodeBetterSerde::BackTab => Self::BackTab,
            KeyCodeBetterSerde::Delete => Self::Delete,
            KeyCodeBetterSerde::Insert => Self::Insert,
            KeyCodeBetterSerde::F { value } => Self::F(value),
            KeyCodeBetterSerde::Char { value } => Self::Char(value),
            KeyCodeBetterSerde::Null => Self::Null,
            KeyCodeBetterSerde::Esc => Self::Esc,
        }
    }
}

impl From<KeyEvent> for KeyEventBetterSerde {
    fn from(value: KeyEvent) -> Self {
        Self {
            code: value.code.into(),
            modifier: value.modifier,
        }
    }
}

impl From<KeyCode> for KeyCodeBetterSerde {
    fn from(value: KeyCode) -> Self {
        match value {
            KeyCode::Backspace => Self::Backspace,
            KeyCode::Left => Self::Left,
            KeyCode::Right => Self::Right,
            KeyCode::Up => Self::Up,
            KeyCode::Down => Self::Down,
            KeyCode::Home => Self::Home,
            KeyCode::End => Self::End,
            KeyCode::PageUp => Self::PageUp,
            KeyCode::PageDown => Self::PageDown,
            KeyCode::BackTab => Self::BackTab,
            KeyCode::Delete => Self::Delete,
            KeyCode::Insert => Self::Insert,
            KeyCode::F(value) => Self::F { value },
            KeyCode::Char(value) => Self::Char { value },
            KeyCode::Null => Self::Null,
            KeyCode::Esc => Self::Esc,
        }
    }
}
