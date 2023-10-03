use bytes::Bytes;
use d_id::client::ClientBuilder;
use d_id::prelude::*;
use http_body_util::Empty;
use d_id::endpoints::resources::voices::get_voices;

#[tokio::main]
async fn main() -> Result<()> {
    let voices = get_voices().await?;

    println!("{:#?}", voices);

    Ok(())
}
