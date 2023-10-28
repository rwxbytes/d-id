// File: animations.rs
// Path: src/endpoints/video/animations.rs

use super::*;

const ANIMATIONS_PATH: &str = "/animations";


#[derive(Serialize, Deserialize, Debug)]
pub struct AnimationRequestBody {
    source_url: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    driver_url: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    result_url: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    webhook: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    user_data: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    face: Option<Face>,
    #[serde(skip_serializing_if = "Option::is_none")]
    config: Option<Config>,
}

impl AnimationRequestBody {
    pub async fn create_animation(&self) -> Result<PostAnimationResponse> {
        let c = ClientBuilder::new()?
            .method(POST)?
            .path(ANIMATIONS_PATH)?
            .header(CONTENT_TYPE, APPLICATION_JSON)?
            .build()?;

        let body = serde_json::to_string(&self)?;

        let resp = c.send_request(Full::<Bytes>::new(body.into())).await?;

        let animation_resp = serde_json::from_slice::<PostAnimationResponse>(&resp.as_ref())?;

        Ok(animation_resp)
    }

}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnimationRequestBodyBuilder {
    source_url: Option<String>,
    driver_url: Option<String>,
    result_url: Option<String>,
    webhook: Option<String>,
    user_data: Option<String>,
    face: Option<Face>,
    config: Option<Config>,
}

impl AnimationRequestBodyBuilder {
    pub fn new() -> Self {
        Self {
            source_url: None,
            driver_url: None,
            result_url: None,
            webhook: None,
            user_data: None,
            face: None,
            config: None,
        }
    }

    pub fn source_url(mut self, source_url: String) -> Self {
        self.source_url = Some(source_url);
        self
    }

    pub fn driver_url(mut self, driver_url: String) -> Self {
        self.driver_url = Some(driver_url);
        self
    }

    pub fn result_url(mut self, result_url: String) -> Self {
        self.result_url = Some(result_url);
        self
    }

    pub fn webhook(mut self, webhook: String) -> Self {
        self.webhook = Some(webhook);
        self
    }

    pub fn user_data(mut self, user_data: String) -> Self {
        self.user_data = Some(user_data);
        self
    }

    pub fn face(mut self, face: Face) -> Self {
        self.face = Some(face);
        self
    }

    pub fn config(mut self, config: Config) -> Self {
        self.config = Some(config);
        self
    }

    pub fn build(self) -> Result<AnimationRequestBody> {
        let source_url = self.source_url.ok_or(RequestBodyBuildError::SourceUrlNotSet)?;

        Ok(AnimationRequestBody {
            source_url,
            driver_url: self.driver_url.unwrap_or_default(),
            result_url: self.result_url.unwrap_or_default(),
            webhook: self.webhook.unwrap_or_default(),
            user_data: self.user_data.unwrap_or_default(),
            face: self.face,
            config: self.config,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostAnimationResponse {
    pub id: String,
    pub object: String,
    pub status: String,
    pub created_by: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Animation {
    pub id: String,
    pub user_id: String,
    pub source_url: String,
    //pub driver_url: String,
    pub status: String,
    //pub created_by: String,
    //pub created_at: String,
    //pub started_at: String,
    pub modified_at: String,
    #[serde(default)]
    pub result_url: String,
    #[serde(default)]
    pub error: Option<AnimationError>,
    //pub webhook: String,
    //pub config: Config,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAnimationsResponse {
    pub animations: Vec<Animation>,
}


pub async fn get_animation(animation_id: &str) -> Result<Animation> {
    let c = ClientBuilder::new()?
        .method(GET)?
        .path(&format!("{}/{}", ANIMATIONS_PATH, animation_id))?
        .header(CONTENT_TYPE, APPLICATION_JSON)?
        .build()?;

    let resp = c.send_request(Empty::<Bytes>::new()).await?;

    let animation_resp = serde_json::from_slice::<Animation>(&resp.as_ref())?;

    Ok(animation_resp)
}

pub async fn get_animations() -> Result<GetAnimationsResponse> {
    let c = ClientBuilder::new()?
        .method(GET)?
        .path(ANIMATIONS_PATH)?
        .header(ACCEPT, APPLICATION_JSON)?
        .build()?;

    let resp = c.send_request(Empty::<Bytes>::new()).await?;

    let animations = serde_json::from_slice::<GetAnimationsResponse>(&resp.as_ref())?;

    Ok(animations)
}

pub async fn delete_animation(animation_id: &str) -> Result<()> {
    let c = ClientBuilder::new()?
        .method(DELETE)?
        .path(&format!("{}/{}", ANIMATIONS_PATH, animation_id))?
        .header(CONTENT_TYPE, APPLICATION_JSON)?
        .build()?;

    let _resp = c.send_request(Empty::<Bytes>::new()).await?;


    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnimationError {
    pub kind: String,
    pub description: String,
}