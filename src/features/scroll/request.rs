use serde::{Deserialize, Serialize};

use crate::bindings::Colour;

#[derive(Serialize, Debug)]
#[serde(tag = "type")]
/// Represents a line position
pub enum ScrollPosition {
    /// Absolute line position
    #[serde(rename = "absolute")]
    Absolute { index: u32 },
    /// Line position relative to cursor (last line)
    #[serde(rename = "relative")]
    Relative { index: i32 },
}

impl ScrollPosition {
    pub fn absolute(index: u32) -> Self {
        Self::Absolute { index }
    }

    pub fn relative(index: i32) -> Self {
        Self::Relative { index }
    }
}

#[derive(Serialize, Clone, Debug)]
#[serde(tag = "type")]
/// A single unit in an entry
pub enum Chunk {
    #[serde(rename = "colour")]
    /// Set colour of text following this
    Colour { value: Colour },
    #[serde(rename = "text")]
    /// Text content
    Text { value: String },
}

impl Chunk {
    pub fn colour(colour: Colour) -> Self {
        Self::Colour { value: colour }
    }

    pub fn text(text: String) -> Self {
        Self::Text { value: text }
    }
}

#[derive(Default, Serialize, Debug)]
pub struct Entry(pub Vec<Chunk>);

impl From<Vec<Chunk>> for Entry {
    fn from(value: Vec<Chunk>) -> Self {
        Self(value)
    }
}

impl From<String> for Entry {
    fn from(value: String) -> Self {
        Self(vec![Chunk::text(value)])
    }
}

impl From<&str> for Entry {
    fn from(value: &str) -> Self {
        value.to_string().into()
    }
}

impl Entry {
    /// Add a single chunk
    pub fn push(&mut self, new: Chunk) {
        self.0.push(new)
    }

    /// Append another entry to this one
    pub fn append(&mut self, other: &mut Self) {
        self.0.append(&mut other.0)
    }

    /// Join two entries to one
    pub fn join(mut self, mut other: Self) -> Self {
        self.append(&mut other);
        self
    }
}

/// A single scroll request
#[derive(Serialize, Debug)]
pub struct ScrollRequest {
    #[serde(flatten)]
    pub content: ScrollRequestVariant,
    pub id: u32,
}

#[derive(Serialize, Debug)]
#[serde(tag = "type", content = "content")]
pub enum ScrollRequestVariant {
    #[serde(rename = "add")]
    /// Add an entry at position
    AddEntry {
        #[serde(flatten)]
        position: ScrollPosition,
        entry: Entry,
    },
    /// Remove an entry with uid
    #[serde(rename = "remove")]
    RemoveEntry { uid: u32 },
    /// Update an entry with uid
    #[serde(rename = "update")]
    UpdateEntry { uid: u32, new: Entry },
    #[serde(rename = "multiple")]
    /// Do multiple request at the same time
    Multiple { requests: Vec<ScrollRequest> },
}

#[derive(Deserialize, Debug)]
pub struct ScrollResponse {
    #[serde(flatten)]
    pub content: ScrollResponseVariant,
    pub id: u32,
}

impl ScrollRequest {
    pub fn new(id: u32, content: ScrollRequestVariant) -> Self {
        Self { content, id }
    }
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum ScrollResponseVariant {
    #[serde(rename = "created")]
    Created { uid: u32 },
    #[serde(rename = "updated")]
    Updated,
    #[serde(rename = "removed")]
    Removed,
    #[serde(rename = "not found")]
    NotFound,
    #[serde(rename = "recieved")]
    Recieved,
    #[serde(rename = "multiple")]
    Multiple { responses: Vec<Self> },
}

impl ScrollResponseVariant {
    /// Returns created uid
    pub fn as_created(&self) -> Option<u32> {
        if let Self::Created { uid } = self {
            Some(*uid)
        } else {
            None
        }
    }

    /// Returns a vector slice of responses
    pub fn as_multiple(&self) -> Option<&[Self]> {
        if let Self::Multiple { responses } = self {
            Some(responses)
        } else {
            None
        }
    }
}
