use crate::errors::AnalyticsError;
use crate::message::MessageKind;
use crate::ruddermessage::RudderMessage;
use std::time::Duration;

pub struct RudderAnalytics {
    pub write_key: String,
    pub data_plane_url: String,
    pub client: reqwest::blocking::Client,
}

impl RudderAnalytics {
    #[must_use]
    pub fn load(write_key: String, data_plane_url: String) -> RudderAnalytics {
        RudderAnalytics {
            write_key,
            data_plane_url,
            client: reqwest::blocking::Client::builder()
                .connect_timeout(Duration::from_secs(10))
                .build()
                .expect("TLS must be supported"),
        }
    }

    /// # Errors
    pub fn send(&self, message: &MessageKind) -> Result<(), AnalyticsError> {
        message.validate()?;

        let path = match message {
            MessageKind::Identify(_) => "/v1/identify",
            MessageKind::Track(_) => "/v1/track",
            MessageKind::Page(_) => "/v1/page",
            MessageKind::Screen(_) => "/v1/screen",
            MessageKind::Group(_) => "/v1/group",
            MessageKind::Alias(_) => "/v1/alias",
        };

        let rudder_message: RudderMessage = message.into();

        let request = self
            .client
            .post(format!("{}{path}", self.data_plane_url))
            .basic_auth(self.write_key.to_string(), Some(""))
            .json(&rudder_message);

        std::thread::spawn(|| {
            let _: Result<_, _> = request.send();
        });

        Ok(())
    }
}
