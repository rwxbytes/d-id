// File: credits.rs
// Path: src/endpoints/resources/credits.rs

use super::*;

use serde::{Deserialize, Serialize};

const CREDITS_PATH: &str = "/credits";

#[derive(Serialize, Deserialize, Debug)]
pub struct CreditResponse {
    credits: Vec<Info>,
    remaining: i64,
    total: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Info {
    owner_id: String,
    expire_at: String,
    created_at: String,
    remaining: i64,
    valid_from: String,
    total: i64,
    product_id: String,
    modified_at: String,
}

pub async fn get_credits() -> Result<CreditResponse> {
    let c = ClientBuilder::new()?
        .method(GET)?
        .path(CREDITS_PATH)?
        .header(CONTENT_TYPE, APPLICATION_JSON)?
        .build()?;

    let resp = c.send_request(Empty::<Bytes>::new()).await?;

    let credits = serde_json::from_slice::<CreditResponse>(&resp.as_ref())?;

    Ok(credits)
}
