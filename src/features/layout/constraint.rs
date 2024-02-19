use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
/// Representing a length
pub struct Constraint {
    /// Base constraint
    base: ConstraintVariant,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Positive offset - adds to the final constraint
    offset_pos: Option<Box<Constraint>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Negative offset - takes away from the final constraint
    offset_neg: Option<Box<Constraint>>,
}

impl Constraint {
    /// Create a new constraint from its fields
    pub fn new(
        base: ConstraintVariant,
        offset_pos: Option<Constraint>,
        offset_neg: Option<Constraint>,
    ) -> Self {
        Self {
            base,
            offset_pos: offset_pos.map(Box::new),
            offset_neg: offset_neg.map(Box::new),
        }
    }
}

impl From<ConstraintVariant> for Constraint {
    fn from(value: ConstraintVariant) -> Self {
        Self::new(value, None, None)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ConstraintVariant {
    /// Maximum length it can take
    #[serde(rename = "max")]
    Max { value: u32 },
    /// Minimum length it can take before skipping rendering
    #[serde(rename = "min")]
    Min { value: u32 },
    /// Absolute length
    #[serde(rename = "length")]
    Length { value: u32 },
    #[serde(rename = "percentage")]
    /// Percentage of width left
    Percentage { value: u32 },
}

impl ConstraintVariant {
    pub fn max(value: u32) -> Self {
        Self::Max { value }
    }

    pub fn min(value: u32) -> Self {
        Self::Min { value }
    }

    pub fn length(value: u32) -> Self {
        Self::Length { value }
    }

    pub fn percentage(value: u32) -> Self {
        Self::Percentage { value }
    }
}
