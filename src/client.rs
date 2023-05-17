use crate::errors::AnalyticsError;
use crate::message::Message;
use crate::utils;
use log::debug;
use std::time::Duration;

// Rudderanalytics client
pub struct RudderAnalytics {
    pub write_key: String,
    pub data_plane_url: String,
    pub client: reqwest::blocking::Client,
}

impl RudderAnalytics {
    /// # Panics
    // Function to initialize the Rudderanalytics client with write-key and data-plane-url
    #[must_use]
    pub fn load(write_key: String, data_plane_url: String) -> RudderAnalytics {
        RudderAnalytics {
            write_key,
            data_plane_url,
            client: reqwest::blocking::Client::builder()
                .connect_timeout(Duration::new(10, 0))
                .build()
                .unwrap(),
        }
    }

    // Function that will receive user event data
    // and after validation
    // modify it to Ruddermessage format and send the event to data plane url
    /// # Errors
    /// # Panics
    #[allow(clippy::too_many_lines)]
    pub fn send(&self, message: &Message) -> Result<(), AnalyticsError> {
        // match the type of event and fetch the proper API path
        let path = match message {
            Message::Identify(identify_message) => {
                // Checking for userId and anonymousId
                if identify_message.user_id.is_none() && identify_message.anonymous_id.is_none() {
                    return Err(AnalyticsError::InvalidRequest);
                }
                "/v1/identify"
            }
            Message::Track(track_message) => {
                // Checking for userId and anonymousId
                if track_message.user_id.is_none() && track_message.anonymous_id.is_none() {
                    return Err(AnalyticsError::InvalidRequest);
                }
                "/v1/track"
            }
            Message::Page(page_message) => {
                // Checking for userId and anonymousId
                if page_message.user_id.is_none() && page_message.anonymous_id.is_none() {
                    return Err(AnalyticsError::InvalidRequest);
                }
                "/v1/page"
            }
            Message::Screen(screen_message) => {
                // Checking for userId and anonymousId
                if screen_message.user_id.is_none() && screen_message.anonymous_id.is_none() {
                    return Err(AnalyticsError::InvalidRequest);
                }
                "/v1/screen"
            }
            Message::Group(group_message) => {
                // Checking for userId and anonymousId
                if group_message.user_id.is_none() && group_message.anonymous_id.is_none() {
                    return Err(AnalyticsError::InvalidRequest);
                }
                "/v1/group"
            }
            Message::Alias(_) => "/v1/alias",
            Message::Batch(_) => "/v1/batch",
        };

        // match the type of event and manipulate the payload to rudder format
        let rudder_message = match message {
            Message::Identify(identify_message) => utils::parse_identify(identify_message),
            Message::Track(track_message) => utils::parse_track(track_message),
            Message::Page(page_message) => utils::parse_page(page_message),
            Message::Screen(screen_message) => utils::parse_screen(screen_message),
            Message::Group(group_message) => utils::parse_group(group_message),
            Message::Alias(alias_message) => utils::parse_alias(alias_message),
            Message::Batch(batch_message) => utils::parse_batch(batch_message),
        };

        // final payload
        debug!("rudder_message: {:#?}", rudder_message);

        let request = self
            .client
            .post(format!("{}{}", self.data_plane_url, path))
            .basic_auth(self.write_key.to_string(), Some(""))
            .json(&rudder_message);

        std::thread::spawn(|| {
            let _: Result<_, _> = request.send();
        });

        Ok(())
    }
}
