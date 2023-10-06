// File: audios.rs
// Path: src/endpoints/resources/audios.rs

use super::*;

const AUDIOS_PATH: &str = "/audios";

#[derive(Serialize, Deserialize, Debug)]
pub struct AudioResponse {
    pub id: String,
    pub url: String,
    pub duration: f64,
}

/// Upload an audio to a temporary storage before creating an animation.
/// Supported mime types: audio/, video/,
/// Storage duration: 24-48H
/// The resulting file is stored as a .wav file in a 16kHz sample rate.
/// The maximum file size is 6MB.
// TODO: Cover all body params [https://docs.d-id.com/reference/upload-an-audio]
pub async  fn upload_audio_by_file(mime_type: &str, path: &str) -> Result<AudioResponse> {
    let mut form = MultipartFormData::new();
    form.add_file(mime_type, "audio", path)?;
    form.end_body()?;

    let c = ClientBuilder::new()?
        .method(POST)?
        .path(AUDIOS_PATH)?
        .header(ACCEPT, APPLICATION_JSON)?
        .header(CONTENT_TYPE, &format!("{}{}", MULTIPART_FORM_DATA_BOUNDARY, form.boundary))?
        .build()?;

    let resp = c.send_request(Full::<Bytes>::new(form.body.into())).await?;

    let json = serde_json::from_slice::<AudioResponse>(&resp.as_ref())?;

    Ok(json)
}


pub async fn delete_audio(id: &str) -> Result<()> {
    let c = ClientBuilder::new()?
        .method(DELETE)?
        .path(&format!("{}/{}", AUDIOS_PATH, id))?
        .header(CONTENT_TYPE, APPLICATION_JSON)?
        .build()?;

    let _resp = c.send_request(Empty::<Bytes>::new()).await?;

    Ok(())
}

// TODO: Implement [https://docs.d-id.com/reference/upload-an-audio1]
