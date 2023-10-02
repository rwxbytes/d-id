pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub const DELETE: &str = "DELETE";
pub const GET: &str = "GET";
pub const POST: &str = "POST";

pub const ACCEPT: &str = "accept";
pub const APPLICATION_JSON: &str = "application/json";
pub const CONTENT_TYPE: &str = "Content-Type";
pub const IMAGE_PNG: &str = "image/png";
