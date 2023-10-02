use bytes::Bytes;
use d_id::client::ClientBuilder;
use d_id::prelude::*;
use http_body_util::Empty;
use d_id::endpoints::resources::credits;

#[tokio::main]
async fn main() -> Result<()> {
    let credits = credits::get_credits().await?;

    println!("{:#?}", credits);

    Ok(())
}
