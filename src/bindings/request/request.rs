use std::path::PathBuf;

use serde::Serialize;
use serde_json::Value;

use crate::{bindings::Discriminator, client::Client};

use super::Subscription;

#[derive(Serialize, Debug, Clone)]
/// A single request to send to the server.
pub struct Request {
    /// recipient
    target: Discriminator,
    /// Content of the request
    pub content: RequestContent,
    /// confirmation identifier
    id: u32,
}

impl Request {
    /// Construct new self with unique ID.
    pub fn new(target: Discriminator, content: RequestContent) -> Self {
        Self {
            target,
            content,
            id: Client::reqid(),
        }
    }

    /// Returns ID of self.
    pub fn id(&self) -> u32 {
        self.id
    }
}

#[derive(Serialize, Clone, PartialEq, Debug)]
#[serde(tag = "type")]
/// Real content of the request.
pub enum RequestContent {
    #[serde(rename = "confirm recieve")]
    /// Confirm that an event has been recieved.
    ConfirmRecieve {
        /// Event ID
        id: u32,
        /// Whether to allow passing of the event or not.
        /// - true = allow passing
        /// - false = capture event
        pass: bool,
    },

    #[serde(rename = "subscribe")]
    /// Add a subscription to a channel with priority.
    Subscribe {
        channel: Subscription,
        priority: Option<u32>,
        component: Option<Discriminator>,
    },

    #[serde(rename = "Unsubscribe")]
    /// Remove a subscription from a channel.
    Unsubscribe {
        channel: Subscription,
        component: Option<Discriminator>,
    },
    #[serde(rename = "set socket")]
    /// Hint server to use this socket for communication.
    ///
    /// This is ran when the client is created.
    SetSocket { path: PathBuf },

    #[serde(rename = "drop")]
    /// Remove a single component.
    Drop { discrim: Option<Discriminator> },

    #[serde(rename = "render")]
    /// Render something to the terminal.
    Render { content: RenderRequest, flush: bool },

    #[serde(rename = "spawn")]
    /// Spawns a new process.
    Spawn {
        command: String,
        args: Vec<String>,
        label: String,
    },

    #[serde(rename = "message")]
    /// Send a message to another component.
    /// If target specifies a space,
    /// all components under that space will recieve the message.
    Message {
        content: String,
        sender: Discriminator,
        target: Discriminator,
    },

    /// Create a new space at a space.
    #[serde(rename = "new space")]
    NewSpace { label: String },

    /// Set focus to a specific space.
    #[serde(rename = "focus at")]
    FocusAt,

    /// Get a state value.
    ///
    /// A state value is intrinsic to the state of the canvas, e.g. dimensions.
    /// And cannot be modified programatically.
    #[serde(rename = "get state")]
    GetState { label: StateValue },

    /// Get value of an entry (variable).
    #[serde(rename = "get entry")]
    GetEntry { label: String },

    /// Remove an entry (variable).
    #[serde(rename = "remove entry")]
    RemoveEntry { label: String },

    /// Set value of an entry (variable). Creates the entry if it does not already exist.
    #[serde(rename = "set entry")]
    SetEntry { label: String, value: Value },

    /// Watch the value of an entry.
    #[serde(rename = "watch")]
    Watch { label: String },

    /// Unwatch the value of an entry.
    #[serde(rename = "unwatch")]
    Unwatch {
        label: String,
        watcher: Discriminator,
    },

    /// Suppress all events from a channel for subscribers with a lower priority
    #[serde(rename = "suppress")]
    Suppress {
        channel: Subscription,
        priority: u32,
    },

    /// Remove a suppression
    #[serde(rename = "unsuppress")]
    Unsuppress { channel: Subscription, id: u32 },
}

#[derive(Serialize, Clone, PartialEq, Eq, Debug)]
#[serde(tag = "type")]
/// A render request to the server.
pub enum RenderRequest {
    /// Change a single character
    #[serde(rename = "set char")]
    SetChar { x: u32, y: u32, c: char },
    /// Change a single character, coloured
    #[serde(rename = "set colouredchar")]
    SetCharColoured {
        x: u32,
        y: u32,
        c: char,
        fg: Colour,
        bg: Colour,
    },
    /// Flush all changes, this is usually not needed
    #[serde(rename = "flush")]
    Flush,
    /// Set cursor looks
    #[serde(rename = "set cursorstyle")]
    SetCursorStyle { style: CursorStyle },
    /// Hide cursor
    #[serde(rename = "hide cursor")]
    HideCursor,
    /// Unhide cursor
    #[serde(rename = "show cursor")]
    ShowCursor,
    /// Clear terminal content
    #[serde(rename = "clear all")]
    ClearAll,

    /// Render multiple items at the same time - guaranteed to be rendered at the same time, and
    /// socket performance is significantly better than sending individual requests.
    #[serde(rename = "render multiple")]
    RenderMultiple { tasks: Vec<Self> },
}

impl RenderRequest {
    /// Create a setchar request
    pub fn setchar(x: u32, y: u32, c: char) -> Self {
        Self::SetChar { x, y, c }
    }

    /// Create a setchar coloured request
    pub fn setchar_coloured(x: u32, y: u32, c: char, fg: Colour, bg: Colour) -> Self {
        Self::SetCharColoured { x, y, c, fg, bg }
    }

    /// Create a setcursor request
    pub fn setcursor(style: CursorStyle) -> Self {
        Self::SetCursorStyle { style }
    }
}

#[derive(Serialize, Clone, Copy, PartialEq, Eq, Debug)]
/// A cursor style.
pub enum CursorStyle {
    #[serde(rename = "blinking bar")]
    BlinkingBar,
    #[serde(rename = "blinking block")]
    BlinkingBlock,
    #[serde(rename = "blinking underline")]
    BlinkingUnderline,
    #[serde(rename = "steady bar")]
    SteadyBar,
    #[serde(rename = "steady block")]
    SteadyBlock,
    #[serde(rename = "steady underline")]
    SteadyUnderline,
}

#[derive(Serialize, Clone, Copy, PartialEq, Eq, Debug)]
#[serde(tag = "type")]
/// Terminal colours.
pub enum Colour {
    #[serde(rename = "black")]
    Black,
    #[serde(rename = "blue")]
    Blue,
    #[serde(rename = "cyan")]
    Cyan,
    #[serde(rename = "green")]
    Green,
    #[serde(rename = "magenta")]
    Magenta,
    #[serde(rename = "red")]
    Red,
    #[serde(rename = "white")]
    White,
    #[serde(rename = "yellow")]
    Yellow,

    #[serde(rename = "lightblack")]
    LightBlack,
    #[serde(rename = "lightblue")]
    LightBlue,
    #[serde(rename = "lightcyan")]
    LightCyan,
    #[serde(rename = "lightgreen")]
    LightGreen,
    #[serde(rename = "lightmagenta")]
    LightMagenta,
    #[serde(rename = "lightred")]
    LightRed,
    #[serde(rename = "lightwhite")]
    LightWhite,
    #[serde(rename = "lightyellow")]
    LightYellow,

    #[serde(rename = "reset")]
    Reset,
    #[serde(rename = "ansi")]
    Ansi { value: u8 },
    #[serde(rename = "rgb")]
    Rgb { red: u8, green: u8, blue: u8 },
}

/// Intrinsic state values
#[derive(Serialize, Clone, PartialEq, Eq, Debug)]
pub enum StateValue {
    #[serde(rename = "focused")]
    Focused,
    #[serde(rename = "is focused")]
    IsFocused,
    #[serde(rename = "term size")]
    TermSize,
    #[serde(rename = "working dir")]
    WorkingDir,
}
