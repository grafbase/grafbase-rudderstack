use thiserror::Error;

#[derive(Debug, Error)]
pub enum AnalyticsError {
    #[error("either user_id or anonymous_id are required")]
    InvalidRequest,
}
