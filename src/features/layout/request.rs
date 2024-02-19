use crate::{bindings::Discriminator, features::common::Direction};
use serde::{Deserialize, Serialize};

use super::{Border, Constraint, Layout};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
/// A single request for ccanvas-layout
pub enum LayoutRequest {
    #[serde(rename = "add")]
    /// Add a new layout item at a location
    Add {
        /// Target location to add to
        at: Vec<Direction>,
        /// Split direction - where the new layout item would be compared to the target
        split: Direction,
        /// Split constraint - left or top
        constraint_1: Constraint,
        /// Split constraint - right or bottom
        constraint_2: Constraint,
        #[serde(skip_serializing_if = "Option::is_none")]
        /// The component discriminator to be displayed in the window
        component: Option<Discriminator>,
        #[serde(skip_serializing_if = "Option::is_none")]
        /// Border of the the layout item
        border: Option<Border>,
    },
    #[serde(rename = "remove")]
    /// Remove a layout item from target location
    Remove { at: Vec<Direction> },
    #[serde(rename = "setlayout")]
    /// Replace/overwrite of target location
    SetLayout { at: Vec<Direction>, layout: Layout },
}
