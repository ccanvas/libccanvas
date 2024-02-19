use crate::bindings::Colour;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
/// Border of a single layout item
pub struct Border {
    /// Colour of the border
    pub colour: Colour,
    #[serde(flatten)]
    /// Style of the border
    pub r#type: BorderType,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum BorderType {
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "rounded")]
    Rounded,
    #[serde(rename = "double")]
    Double,
    #[serde(rename = "thick")]
    Thick,
    #[serde(rename = "custom")]
    Custom {
        left: char,
        topleft: char,
        top: char,
        topright: char,
        right: char,
        bottomright: char,
        bottom: char,
        bottomleft: char,
    },
}

impl Border {
    pub fn new(colour: Colour, r#type: BorderType) -> Self {
        Self { colour, r#type }
    }
}
