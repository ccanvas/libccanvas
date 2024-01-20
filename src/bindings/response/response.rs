use serde::Deserialize;
use serde_json::Value;

use crate::bindings::Discriminator;

use super::EventVariant;

#[derive(Deserialize, Clone, PartialEq, Debug)]
/// A single response to send to the server.
pub struct Response {
    /// the content of the response
    pub content: ResponseContent,

    /// send a confirmation to the server using this id
    /// to confirm recieved
    pub id: u32,

    /// request id for confirmation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<u32>,
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
#[serde(tag = "type")]
/// Real content of the response.
pub enum ResponseContent {
    /// Request could not be delivered to specified recipient.
    #[serde(rename = "undelivered")]
    Undelivered,

    /// This is an event channel item.
    #[serde(rename = "event")]
    Event { content: EventVariant },

    /// Error from a request.
    #[serde(rename = "error")]
    Error { content: ResponseError },

    /// Success from a request.
    #[serde(rename = "success")]
    Success { content: ResponseSuccess },
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
#[serde(tag = "type")]
/// Error variant of responses.
pub enum ResponseError {
    /// Specified component could not be found.
    #[serde(rename = "component not found")]
    ComponentNotFound,
    /// Spawning a component failed.
    #[serde(rename = "spawn failed")]
    SpawnFailed,
    /// Value entry could not be found.
    #[serde(rename = "entry not found")]
    EntryNotFound,
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
#[serde(tag = "type")]
/// Success variant of responses
pub enum ResponseSuccess {
    /// Subscription to channel added.
    #[serde(rename = "subscribe added")]
    SubscribeAdded,

    /// Subscription to channel removed.
    #[serde(rename = "subscribe removed")]
    SubscribeRemoved,

    /// Listener socket has been set, returns the discriminator of the current process.
    #[serde(rename = "listener set")]
    ListenerSet { discrim: Discriminator },

    /// Target component has been dropped.
    #[serde(rename = "dropped")]
    Dropped,

    /// Render task has been completed.
    #[serde(rename = "rendered")]
    Rendered,

    /// New process spawned, return the discriminator of the new process.
    #[serde(rename = "spawned")]
    Spawned { discrim: Discriminator },

    /// Message has been delivered to target component.
    #[serde(rename = "message delivered")]
    MessageDelivered,

    /// New space created, return the discriminator of the new space.
    #[serde(rename = "space created")]
    SpaceCreated { discrim: Discriminator },

    /// Focus has been changed.
    #[serde(rename = "focus changed")]
    FocusChanged,

    /// Returned a requested value.
    #[serde(rename = "value")]
    Value { value: Value },

    /// Value has been changed.
    #[serde(rename = "value set")]
    ValueSet,

    /// Value has been unset.
    #[serde(rename = "removed value")]
    RemovedValue,

    /// Now watching a value.
    #[serde(rename = "watching")]
    Watching,

    /// Unwatched a value.
    #[serde(rename = "unwatched")]
    Unwatched,

    /// Suppressed a channel, returning the suppressor id.
    #[serde(rename = "suppressed")]
    Suppressed { id: u32 },

    /// Suppressor revoked.
    #[serde(rename = "unsuppressed")]
    Unsuppressed,
}
