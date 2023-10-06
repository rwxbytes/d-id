use d_id::prelude::*;
use d_id::endpoints::resources::audios::{upload_audio_by_file, delete_audio};

#[tokio::main]
async fn main() -> Result<()> {
    let aud = upload_audio_by_file("audio/mp3", "tyger_1.mp3").await?;
    println!("{:?}", aud);
    let _del = delete_audio(&aud.id).await?;


    Ok(())
}
