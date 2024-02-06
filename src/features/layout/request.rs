use crate::{bindings::Discriminator, features::common::Direction};
use serde::{Deserialize, Serialize};

use super::{Border, Constraint, Layout};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum LayoutRequest {
    #[serde(rename = "add")]
    Add {
        at: Vec<Direction>,
        split: Direction,
        constraint_1: Constraint,
        constraint_2: Constraint,
        #[serde(skip_serializing_if = "Option::is_none")]
        component: Option<Discriminator>,
        #[serde(skip_serializing_if = "Option::is_none")]
        border: Option<Border>,
    },
    #[serde(rename = "remove")]
    Remove { at: Vec<Direction> },
    #[serde(rename = "setlayout")]
    SetLayout { at: Vec<Direction>, layout: Layout },
}
