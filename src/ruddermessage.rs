use chrono::{DateTime, Utc};
use ijson::IValue;
use serde::{Deserialize, Serialize};

/// An enum containing all values which may be sent to `RudderStack`'s API.
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RudderMessage {
    Identify(Identify),
    Track(Track),
    Page(Page),
    Screen(Screen),
    Group(Group),
    Alias(Alias),
}

/// An identify event.
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
#[serde_with::skip_serializing_none]
pub struct Identify {
    /// The user id associated with this message.
    pub user_id: Option<String>,

    /// The anonymous user id associated with this message.
    pub anonymous_id: Option<String>,

    /// The traits to assign to the user.
    pub traits: Option<IValue>,

    /// The originalTimestamp associated with this message.
    pub original_timestamp: DateTime<Utc>,

    /// sent at timestamp
    pub sent_at: DateTime<Utc>,

    /// Context associated with this message.
    pub context: Option<IValue>,

    /// Integrations to route this message to.
    pub integrations: Option<IValue>,

    /// Type of method
    pub r#type: String,

    /// channel in payload
    pub channel: String,
}

/// A track event.
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
#[serde_with::skip_serializing_none]
pub struct Track {
    /// The user id associated with this message.
    pub user_id: Option<String>,

    /// The anonymous user id associated with this message.
    pub anonymous_id: Option<String>,

    /// The name of the event being tracked.
    pub event: String,

    /// The properties associated with the event.
    pub properties: Option<IValue>,

    /// The originalTimestamp associated with this message.
    pub original_timestamp: DateTime<Utc>,

    /// sent at timestamp
    pub sent_at: DateTime<Utc>,

    /// Context associated with this message.
    pub context: Option<IValue>,

    /// Integrations to route this message to.
    pub integrations: Option<IValue>,

    // Type of method
    pub r#type: String,

    /// channel in payload
    pub channel: String,
}

/// A page event.
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
#[serde_with::skip_serializing_none]
pub struct Page {
    /// The user id associated with this message.
    pub user_id: Option<String>,

    /// The anonymous user id associated with this message.
    pub anonymous_id: Option<String>,

    /// The name of the page being tracked.
    pub name: String,

    /// The properties associated with the event.
    pub properties: Option<IValue>,

    /// The originalTimestamp associated with this message.
    pub original_timestamp: DateTime<Utc>,

    /// sent at timestamp
    pub sent_at: DateTime<Utc>,

    /// Context associated with this message.
    pub context: Option<IValue>,

    /// Integrations to route this message to.
    pub integrations: Option<IValue>,

    // Type of method
    pub r#type: String,

    /// channel in payload
    pub channel: String,
}

/// A screen event.
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
#[serde_with::skip_serializing_none]
pub struct Screen {
    /// The user id associated with this message.
    pub user_id: Option<String>,

    /// The anonymous user id associated with this message.
    pub anonymous_id: Option<String>,

    /// The name of the screen being tracked.
    pub name: String,

    /// The properties associated with the event.
    pub properties: Option<IValue>,

    /// The originalTimestamp associated with this message.
    pub original_timestamp: DateTime<Utc>,

    /// sent at timestamp
    pub sent_at: DateTime<Utc>,

    /// Context associated with this message.
    pub context: Option<IValue>,

    /// Integrations to route this message to.
    pub integrations: Option<IValue>,

    // Type of method
    pub r#type: String,

    /// channel in payload
    pub channel: String,
}

/// A group event.
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
#[serde_with::skip_serializing_none]
pub struct Group {
    /// The user id associated with this message.
    pub user_id: Option<String>,

    /// The anonymous user id associated with this message.
    pub anonymous_id: Option<String>,

    /// The group the user is being associated with.
    pub group_id: String,

    /// The traits to assign to the group.
    pub traits: Option<IValue>,

    /// The originalTimestamp associated with this message.
    pub original_timestamp: DateTime<Utc>,

    /// sent at timestamp
    pub sent_at: DateTime<Utc>,

    /// Context associated with this message.
    pub context: Option<IValue>,

    /// Integrations to route this message to.
    pub integrations: Option<IValue>,

    // Type of method
    pub r#type: String,

    /// channel in payload
    pub channel: String,
}

/// An alias event.
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
#[serde_with::skip_serializing_none]
pub struct Alias {
    /// The user id associated with this message.
    pub user_id: String,

    /// The user's previous ID.
    pub previous_id: String,

    /// The traits to assign to the alias.
    pub traits: Option<IValue>,

    /// The originalTimestamp associated with this message.
    pub original_timestamp: DateTime<Utc>,

    /// sent at timestamp
    pub sent_at: DateTime<Utc>,

    /// Context associated with this message.
    pub context: Option<IValue>,

    /// Integrations to route this message to.
    pub integrations: Option<IValue>,

    // Type of method
    pub r#type: String,

    /// channel in payload
    pub channel: String,
}
