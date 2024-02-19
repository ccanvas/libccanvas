use std::{
    collections::HashMap,
    sync::{Mutex, OnceLock},
};

use tokio::sync::oneshot;

use crate::{
    bindings::Discriminator,
    client::Client,
    features::scroll::{ScrollRequest, SCROLL_REQ},
};

use super::{Entry, ScrollPosition, ScrollRequestVariant, ScrollResponseVariant, SCROLL_CONFIRMS};

static ID: OnceLock<Mutex<u32>> = OnceLock::new();

/// Handle to a ccanvas scroll component
pub struct ScrollComponent {
    discrim: Discriminator,
    client: Client,
}

impl ScrollComponent {
    /// Create a handle from client and discrim
    pub fn new(discrim: Discriminator, client: Client) -> Self {
        Self { discrim, client }
    }

    fn get_id(&self) -> u32 {
        let mut id = ID.get_or_init(|| Mutex::new(0)).lock().unwrap();
        *id += 1;
        *id
    }

    async fn send(&self, content: ScrollRequestVariant) -> Option<ScrollResponseVariant> {
        let (sender, reciever) = oneshot::channel();

        let id = self.get_id();

        if self
            .client
            .message(
                self.discrim.clone(),
                serde_json::to_value(ScrollRequest::new(id, content)).unwrap(),
                SCROLL_REQ.to_string(),
            )
            .await
            .is_undelivered()
        {
            return None;
        }

        SCROLL_CONFIRMS
            .get_or_init(|| std::sync::Mutex::new(HashMap::new()))
            .lock()
            .unwrap()
            .insert(id, sender);

        reciever.await.ok()
    }
}

impl ScrollComponent {
    /// Add new entry to scroll at location
    pub async fn add(
        &self,
        position: ScrollPosition,
        entry: Entry,
    ) -> Option<ScrollResponseVariant> {
        self.send(ScrollRequestVariant::AddEntry { position, entry })
            .await
    }

    /// Add new entry to end
    pub async fn push(&self, entry: Entry) -> Option<ScrollResponseVariant> {
        self.add(ScrollPosition::relative(0), entry).await
    }

    /// Update an existing entry with uid
    pub async fn update(&self, uid: u32, new: Entry) -> Option<ScrollResponseVariant> {
        self.send(ScrollRequestVariant::UpdateEntry { uid, new })
            .await
    }

    /// Remove an existing entry with uid
    pub async fn remove(&self, uid: u32) -> Option<ScrollResponseVariant> {
        self.send(ScrollRequestVariant::RemoveEntry { uid }).await
    }
}
