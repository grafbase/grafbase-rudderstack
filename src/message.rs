use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::errors::AnalyticsError;

/// An enum containing all values which may be sent to `RudderStack`'s API.
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Message {
    Identify(Identify),
    Track(Track),
    Page(Page),
    Screen(Screen),
    Group(Group),
    Alias(Alias),
    Batch(Batch),
}

impl Message {
    /// # Errors
    pub fn validate(&self) -> Result<(), AnalyticsError> {
        let valid = match self {
            Message::Identify(identify_message) => {
                identify_message.user_id.is_some() || identify_message.anonymous_id.is_some()
            }
            Message::Track(track_message) => track_message.user_id.is_some() || track_message.anonymous_id.is_some(),
            Message::Page(page_message) => page_message.user_id.is_some() || page_message.anonymous_id.is_some(),
            Message::Screen(screen_message) => {
                screen_message.user_id.is_some() || screen_message.anonymous_id.is_some()
            }
            Message::Group(group_message) => group_message.user_id.is_some() || group_message.anonymous_id.is_some(),
            Message::Alias(_) | Message::Batch(_) => true,
        };
        if valid {
            Ok(())
        } else {
            Err(AnalyticsError::InvalidRequest)
        }
    }
}

/// An identify event.
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct Identify {
    /// The user id associated with this message.
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,

    /// The anonymous user id associated with this message.
    #[serde(rename = "anonymousId", skip_serializing_if = "Option::is_none")]
    pub anonymous_id: Option<String>,

    /// The traits to assign to the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traits: Option<Value>,

    /// The timestamp associated with this message.
    #[serde(rename = "originalTimestamp", skip_serializing_if = "Option::is_none")]
    pub original_timestamp: Option<DateTime<Utc>>,

    /// Context associated with this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Value>,

    /// Integrations to route this message to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integrations: Option<Value>,
}

/// A track event.
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct Track {
    /// The user id associated with this message.
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,

    /// The anonymous user id associated with this message.
    #[serde(rename = "anonymousId", skip_serializing_if = "Option::is_none")]
    pub anonymous_id: Option<String>,

    /// The name of the event being tracked.
    pub event: String,

    /// The properties associated with the event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Value>,

    /// The timestamp associated with this message.
    #[serde(rename = "originalTimestamp", skip_serializing_if = "Option::is_none")]
    pub original_timestamp: Option<DateTime<Utc>>,

    /// Context associated with this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Value>,

    /// Integrations to route this message to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integrations: Option<Value>,
}

/// A page event.
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct Page {
    /// The user id associated with this message.
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,

    /// The anonymous user id associated with this message.
    #[serde(rename = "anonymousId", skip_serializing_if = "Option::is_none")]
    pub anonymous_id: Option<String>,

    /// The name of the page being tracked.
    pub name: String,

    /// The properties associated with the event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Value>,

    /// The timestamp associated with this message.
    #[serde(rename = "originalTimestamp", skip_serializing_if = "Option::is_none")]
    pub original_timestamp: Option<DateTime<Utc>>,

    /// Context associated with this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Value>,

    /// Integrations to route this message to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integrations: Option<Value>,
}

/// A screen event.
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct Screen {
    /// The user id associated with this message.
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,

    /// The anonymous user id associated with this message.
    #[serde(rename = "anonymousId", skip_serializing_if = "Option::is_none")]
    pub anonymous_id: Option<String>,

    /// The name of the screen being tracked.
    pub name: String,

    /// The properties associated with the event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Value>,

    /// The timestamp associated with this message.
    #[serde(rename = "originalTimestamp", skip_serializing_if = "Option::is_none")]
    pub original_timestamp: Option<DateTime<Utc>>,

    /// Context associated with this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Value>,

    /// Integrations to route this message to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integrations: Option<Value>,
}

/// A group event.
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct Group {
    /// The user id associated with this message.
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,

    /// The anonymous user id associated with this message.
    #[serde(rename = "anonymousId", skip_serializing_if = "Option::is_none")]
    pub anonymous_id: Option<String>,

    /// The group the user is being associated with.
    #[serde(rename = "groupId")]
    pub group_id: String,

    /// The traits to assign to the group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traits: Option<Value>,

    /// The timestamp associated with this message.
    #[serde(rename = "originalTimestamp", skip_serializing_if = "Option::is_none")]
    pub original_timestamp: Option<DateTime<Utc>>,

    /// Context associated with this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Value>,

    /// Integrations to route this message to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integrations: Option<Value>,
}

/// An alias event.
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct Alias {
    /// The user id associated with this message.
    #[serde(rename = "userId")]
    pub user_id: String,

    /// The user's previous ID.
    #[serde(rename = "previousId")]
    pub previous_id: String,

    /// The traits to assign to the alias.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traits: Option<Value>,

    /// The timestamp associated with this message.
    #[serde(rename = "originalTimestamp", skip_serializing_if = "Option::is_none")]
    pub original_timestamp: Option<DateTime<Utc>>,

    /// Context associated with this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Value>,

    /// Integrations to route this message to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integrations: Option<Value>,
}

/// A batch of events.
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct Batch {
    /// The batch of messages to send.
    pub messages: Vec<BatchMessage>,

    /// Context associated with this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Value>,

    /// Integrations to route this message to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integrations: Option<Value>,

    /// The timestamp associated with this message.
    #[serde(rename = "originalTimestamp", skip_serializing_if = "Option::is_none")]
    pub original_timestamp: Option<DateTime<Utc>>,
}

#[allow(clippy::module_name_repetitions)]
/// An enum containing all messages which may be placed inside a batch.
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum BatchMessage {
    #[serde(rename = "identify")]
    Identify(Identify),
    #[serde(rename = "track")]
    Track(Track),
    #[serde(rename = "page")]
    Page(Page),
    #[serde(rename = "screen")]
    Screen(Screen),
    #[serde(rename = "group")]
    Group(Group),
    #[serde(rename = "alias")]
    Alias(Alias),
}
