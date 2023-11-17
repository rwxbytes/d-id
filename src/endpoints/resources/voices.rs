// File: voices.rs
// Path: src/endpoints/resources/voices.rs

use super::*;

const VOICES_PATH: &str = "/tts/voices";

pub type Voices = Vec<Voice>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Voice {
    id: String,
    name: String,
    gender: String,
    locale: String,
    language: String,
    access: String,
    provider: String,
    //styles: Vec<Option<String>>,
}

pub async fn get_voices() -> Result<Voices> {
    let c = ClientBuilder::new()?
        .method(GET)?
        .path(VOICES_PATH)?
        .header(CONTENT_TYPE, APPLICATION_JSON)?
        .build()?;

    let resp = c.send_request(Empty::<Bytes>::new()).await?;

    let voices = serde_json::from_slice::<Voices>(&resp.as_ref())?;

    Ok(voices)
}
