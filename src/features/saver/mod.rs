use serde::{Deserialize, Serialize};

use crate::{bindings::Discriminator, client::Client};

pub const SAVER_DIMENSION_LABEL: &str = "!ccanvas-saver-dimensions";
pub const SAVER_ISON_LABEL: &str = "!ccanvas-saver-ison";

#[derive(Serialize, Deserialize)]
pub struct SaverDimension {
    width: u32,
    height: u32,
}

impl SaverDimension {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

impl Client {
    /// Enable screen saver with a specific dimension.
    pub async fn enable_saver(&self, min_width: u32, min_height: u32) {
        self.set(
            SAVER_DIMENSION_LABEL.to_string(),
            Discriminator::master(),
            serde_json::to_value(SaverDimension::new(min_width, min_height)).unwrap(),
        )
        .await;
    }

    /// Disable screeen saver.
    pub async fn disable_saver(&self) {
        self.remove(SAVER_DIMENSION_LABEL.to_string(), Discriminator::master())
            .await;
    }

    /// Returns true if the overlay is currently drawn.
    pub async fn saver_ison(&self) -> bool {
        self.get(SAVER_ISON_LABEL.to_string(), Discriminator::master())
            .await
            .map(|value| serde_json::from_value(value).unwrap_or(false))
            .unwrap_or(false)
    }
}
