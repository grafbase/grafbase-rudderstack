use crate::errors::AnalyticsError;
use crate::message::MessageKind;
use crate::ruddermessage::RudderMessage;
use std::time::Duration;

// Rudderanalytics client
pub struct RudderAnalytics {
    pub write_key: String,
    pub data_plane_url: String,
    pub client: reqwest::blocking::Client,
}

impl RudderAnalytics {
    // Function to initialize the Rudderanalytics client with write-key and data-plane-url
    #[must_use]
    pub fn load(write_key: String, data_plane_url: String) -> RudderAnalytics {
        RudderAnalytics {
            write_key,
            data_plane_url,
            client: reqwest::blocking::Client::builder()
                .connect_timeout(Duration::new(10, 0))
                .build()
                .expect("TLS must be supported"),
        }
    }

    // Function that will receive user event data
    // and after validation
    // modify it to Ruddermessage format and send the event to data plane url
    /// # Errors
    #[allow(clippy::too_many_lines)]
    pub fn send(&self, message: &MessageKind) -> Result<(), AnalyticsError> {
        message.validate()?;

        // match the type of event and fetch the proper API path
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
