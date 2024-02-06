use crate::bindings::Colour;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Border {
    pub colour: Colour,
    #[serde(flatten)]
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
