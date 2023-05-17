//! Errors which may arise from this crate.
use thiserror::Error;

/// An enum of errors this crate may produce. These are compatible with
/// `failure` errors.
#[derive(Debug, Error)]
pub enum AnalyticsError {
    /// The given message is too large to be sent to RudderStack's API.
    #[error("message too large")]
    MessageTooLarge,

    #[error("either user_id or anonymous_id are required")]
    InvalidRequest,
}
