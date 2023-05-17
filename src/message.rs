use crate::errors::AnalyticsError;
use crate::ruddermessage::{
    Alias as Rudderalias, Group as Ruddergroup, Identify as Rudderidentify, Page as Rudderpage, RudderMessage,
    Screen as Rudderscreen, Track as Ruddertrack,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

// constants and reserved keywords
const CHANNEL: &str = "server";

/// An enum containing all values which may be sent to `RudderStack`'s API.
#[allow(clippy::module_name_repetitions)]
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessageKind {
    Identify(Identify),
    Track(Track),
    Page(Page),
    Screen(Screen),
    Group(Group),
    Alias(Alias),
}

pub trait Message {
    fn get_timings(&self) -> (DateTime<Utc>, DateTime<Utc>) {
        let sent_at = Utc::now();
        let original_timestamp = self.get_original_timestamp().unwrap_or(sent_at);
        (sent_at, original_timestamp)
    }
    fn get_original_timestamp(&self) -> Option<DateTime<Utc>>;
    fn get_user_id(&self) -> Option<&str>;
    fn get_anonymous_id(&self) -> Option<&str>;
    /// # Errors
    fn validate(&self) -> Result<(), AnalyticsError> {
        if self.get_user_id().is_some() || self.get_anonymous_id().is_some() {
            Ok(())
        } else {
            Err(AnalyticsError::InvalidRequest)
        }
    }
}

impl Message for Identify {
    fn get_original_timestamp(&self) -> Option<DateTime<Utc>> {
        self.original_timestamp
    }

    fn get_user_id(&self) -> Option<&str> {
        self.user_id.as_deref()
    }

    fn get_anonymous_id(&self) -> Option<&str> {
        self.anonymous_id.as_deref()
    }
}

impl Message for Track {
    fn get_original_timestamp(&self) -> Option<DateTime<Utc>> {
        self.original_timestamp
    }

    fn get_user_id(&self) -> Option<&str> {
        self.user_id.as_deref()
    }

    fn get_anonymous_id(&self) -> Option<&str> {
        self.anonymous_id.as_deref()
    }
}

impl Message for Page {
    fn get_original_timestamp(&self) -> Option<DateTime<Utc>> {
        self.original_timestamp
    }

    fn get_user_id(&self) -> Option<&str> {
        self.user_id.as_deref()
    }

    fn get_anonymous_id(&self) -> Option<&str> {
        self.anonymous_id.as_deref()
    }
}

impl Message for Screen {
    fn get_original_timestamp(&self) -> Option<DateTime<Utc>> {
        self.original_timestamp
    }

    fn get_user_id(&self) -> Option<&str> {
        self.user_id.as_deref()
    }

    fn get_anonymous_id(&self) -> Option<&str> {
        self.anonymous_id.as_deref()
    }
}

impl Message for Group {
    fn get_original_timestamp(&self) -> Option<DateTime<Utc>> {
        self.original_timestamp
    }

    fn get_anonymous_id(&self) -> Option<&str> {
        None
    }
    fn get_user_id(&self) -> Option<&str> {
        None
    }
    fn validate(&self) -> Result<(), AnalyticsError> {
        Ok(())
    }
}

impl Message for Alias {
    fn get_original_timestamp(&self) -> Option<DateTime<Utc>> {
        self.original_timestamp
    }

    fn get_anonymous_id(&self) -> Option<&str> {
        None
    }
    fn get_user_id(&self) -> Option<&str> {
        None
    }
    fn validate(&self) -> Result<(), AnalyticsError> {
        Ok(())
    }
}

impl MessageKind {
    /// # Errors
    pub fn validate(&self) -> Result<(), AnalyticsError> {
        match self {
            MessageKind::Identify(message) => message.validate(),
            MessageKind::Track(message) => message.validate(),
            MessageKind::Page(message) => message.validate(),
            MessageKind::Screen(message) => message.validate(),
            MessageKind::Group(message) => message.validate(),
            MessageKind::Alias(message) => message.validate(),
        }
    }
}

impl From<&MessageKind> for RudderMessage {
    fn from(message: &MessageKind) -> Self {
        match message {
            MessageKind::Identify(identify_message) => {
                let message = &identify_message;
                let (sent_at, original_timestamp) = message.get_timings();

                RudderMessage::Identify(Rudderidentify {
                    user_id: message.user_id.clone(),
                    anonymous_id: message.anonymous_id.clone(),
                    traits: message.traits.clone(),
                    original_timestamp,
                    sent_at,
                    integrations: message.integrations.clone(),
                    context: message.context.clone(),
                    r#type: String::from("identify"),
                    channel: CHANNEL.to_string(),
                })
            }
            MessageKind::Track(track_message) => {
                let message = &track_message;
                let (sent_at, original_timestamp) = message.get_timings();

                RudderMessage::Track(Ruddertrack {
                    user_id: message.user_id.clone(),
                    anonymous_id: message.anonymous_id.clone(),
                    event: message.event.clone(),
                    properties: message.properties.clone(),
                    original_timestamp,
                    sent_at,
                    integrations: message.integrations.clone(),
                    context: message.context.clone(),
                    r#type: String::from("track"),
                    channel: CHANNEL.to_string(),
                })
            }
            MessageKind::Page(page_message) => {
                let message = &page_message;
                let (sent_at, original_timestamp) = message.get_timings();

                RudderMessage::Page(Rudderpage {
                    user_id: message.user_id.clone(),
                    anonymous_id: message.anonymous_id.clone(),
                    name: message.name.clone(),
                    properties: message.properties.clone(),
                    original_timestamp,
                    sent_at,
                    integrations: message.integrations.clone(),
                    context: message.context.clone(),
                    r#type: String::from("page"),
                    channel: CHANNEL.to_string(),
                })
            }
            MessageKind::Screen(screen_message) => {
                let message = &screen_message;
                let (sent_at, original_timestamp) = message.get_timings();

                RudderMessage::Screen(Rudderscreen {
                    user_id: message.user_id.clone(),
                    anonymous_id: message.anonymous_id.clone(),
                    name: message.name.clone(),
                    properties: message.properties.clone(),
                    original_timestamp,
                    sent_at,
                    integrations: message.integrations.clone(),
                    context: message.context.clone(),
                    r#type: String::from("screen"),
                    channel: CHANNEL.to_string(),
                })
            }
            MessageKind::Group(group_message) => {
                let (sent_at, original_timestamp) = group_message.get_timings();

                RudderMessage::Group(Ruddergroup {
                    user_id: group_message.user_id.clone(),
                    anonymous_id: group_message.anonymous_id.clone(),
                    group_id: group_message.group_id.clone(),
                    traits: group_message.traits.clone(),
                    original_timestamp,
                    sent_at,
                    integrations: group_message.integrations.clone(),
                    context: group_message.context.clone(),
                    r#type: String::from("group"),
                    channel: CHANNEL.to_string(),
                })
            }
            MessageKind::Alias(alias_message) => {
                let (sent_at, original_timestamp) = alias_message.get_timings();

                RudderMessage::Alias(Rudderalias {
                    user_id: alias_message.user_id.clone(),
                    previous_id: alias_message.previous_id.clone(),
                    traits: alias_message.traits.clone(),
                    original_timestamp,
                    sent_at,
                    integrations: alias_message.integrations.clone(),
                    context: alias_message.context.clone(),
                    r#type: String::from("alias"),
                    channel: CHANNEL.to_string(),
                })
            }
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
