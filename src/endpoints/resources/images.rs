// File: images.rs
// Path: src/endpoints/resources/voices.rs

use super::*;

const IMAGES_PATH: &str = "/images";

// This temporary URL should be provided when creating an animation via the /animations endpoint.
#[derive(Serialize, Deserialize, Debug)]
pub struct ImageResponse {
    pub id: String,
    pub url: String,
}

/// Upload an image to a temporary storage before creating an animation.
/// Supported mime types: image/jpeg,image/png
/// Storage duration: 24-48H
// TODO: Cover all body params [https://docs.d-id.com/reference/upload-an-image]
pub async fn upload_image_by_file(path: &str) -> Result<ImageResponse> {
    let mime_subtype = path.split(".").last().ok_or("Invalid file path")?;
    let mut form = MultipartFormData::new();
    form.add_file(&format!("image/{}", mime_subtype), "image", path)?;
    form.end_body()?;

    let c = ClientBuilder::new()?
        .method(POST)?
        .path(IMAGES_PATH)?
        .header(ACCEPT, APPLICATION_JSON)?
        .header(CONTENT_TYPE, &format!("{}{}", MULTIPART_FORM_DATA_BOUNDARY, form.boundary))?
        .build()?;

    let resp = c.send_request(Full::<Bytes>::new(form.body.into())).await?;

    let json = serde_json::from_slice::<ImageResponse>(&resp.as_ref())?;

    Ok(json)
}

pub async fn delete_image(id: &str) -> Result<()> {
    let c = ClientBuilder::new()?
        .method(DELETE)?
        .path(&format!("{}/{}", IMAGES_PATH, id))?
        .header(CONTENT_TYPE, APPLICATION_JSON)?
        .build()?;

    let _resp = c.send_request(Empty::<Bytes>::new()).await?;

    Ok(())
}

