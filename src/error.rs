use serde::Deserialize;
use std::fmt;

#[derive(Debug, Deserialize)]
pub struct ApiResponseError {
    pub kind: String,
    pub description: String,
    pub details: Option<String>,
}

// Todo: Handle details - [Celeberity Decetion error is not handled]
impl fmt::Display for ApiResponseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "kind: {}, description: {}, details: {:?}",
            self.kind, self.description, self.details
        )
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Client build error: {0}")]
    ClientBuildError(String),
    #[error("{:?}", .0)]
    ClientSendRequestError(serde_json::Value),
}

#[derive(thiserror::Error, Debug)]
pub enum RequestBodyBuildError {
    #[error("source url must be set")]
    SourceUrlNotSet,
    #[error("script must be set")]
    ScriptNotSet,
    #[error("presenter id must be set")]
    PresenterIdNotSet,
}
