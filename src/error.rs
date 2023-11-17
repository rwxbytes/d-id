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
