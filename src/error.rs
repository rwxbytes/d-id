use serde::Deserialize;
use std::fmt;

#[derive(Debug, Deserialize)]
pub struct ApiResponseError {
    pub kind: String,
    pub description: String,
    pub details: String,
}

impl fmt::Display for ApiResponseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "kind: {}, description: {}, details: {}",
            self.kind, self.description, self.details
        )
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Client build error: {0}")]
    ClientBuildError(String),
    #[error("{:?}", .0)]
    ClientSendRequestError(ApiResponseError),
}
