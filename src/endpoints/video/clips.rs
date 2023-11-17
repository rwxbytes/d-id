// File: clips.rs
// Path: src/endpoints/video/clips.rs

use super::*;

const CLIPS_PATH: &str = "/clips";
const PRESENTERS_PATH: &str = "/presenters";
const DRIVERS_PATH: &str = "/drivers";

#[derive(Serialize, Deserialize, Debug)]
pub struct GetPresentersResponse {
    presenters: Vec<Presenter>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Presenter {
    presenter_id: String,
    driver_id: String,
    gender: String,
    owner_id: String,
    preview_url: String,
    modified_at: String,
    //video_url: String,
}

pub async fn get_presenters() -> Result<GetPresentersResponse> {
    let c = ClientBuilder::new()?
        .method(GET)?
        .path(&format!("{}{}", CLIPS_PATH, PRESENTERS_PATH))?
        .header(CONTENT_TYPE, APPLICATION_JSON)?
        .build()?;

    let resp = c.send_request(Empty::<Bytes>::new()).await?;

    let presenters = serde_json::from_slice::<GetPresentersResponse>(&resp.as_ref())?;

    Ok(presenters)
}

pub async fn get_presenter(id: &str) -> Result<Presenter> {
    let c = ClientBuilder::new()?
        .method(GET)?
        .path(&format!("{}{}/{}", CLIPS_PATH, PRESENTERS_PATH, id))?
        .header(CONTENT_TYPE, APPLICATION_JSON)?
        .build()?;

    let resp = c.send_request(Empty::<Bytes>::new()).await?;

    let presenter = serde_json::from_slice::<Presenter>(&resp.as_ref())?;

    Ok(presenter)
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ClipRequestBody {
    presenter_id: String,
    script: Script,
    #[serde(skip_serializing_if = "String::is_empty")]
    driver_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    config: Option<Config>,
    #[serde(skip_serializing_if = "String::is_empty")]
    created_by: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    presenter_config: Option<PresenterConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    background: Option<Background>,
    #[serde(skip_serializing_if = "String::is_empty")]
    user_data: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    webhook: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    result_url: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    raw_result_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    persist: Option<bool>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PresenterConfig{
    crop: Crop,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Crop {
    r#type: String,
    rectangle: Rectangle,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Rectangle {
    bottom: i64,
    left: i64,
    right: i64,
    top: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Background {
    color: String,
}

impl ClipRequestBody {
    pub async fn create(&self) -> Result<CreateClipResponse> {
        let c = ClientBuilder::new()?
            .method(POST)?
            .path(CLIPS_PATH)?
            .header(CONTENT_TYPE, APPLICATION_JSON)?
            .header(ACCEPT, APPLICATION_JSON)?
            .build()?;

        let resp = c.send_request(Full::<Bytes>::new(serde_json::to_string(&self)?.into())).await?;

        let clip_resp = serde_json::from_slice::<CreateClipResponse>(&resp.as_ref())?;

        Ok(clip_resp)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClipRequestBodyBuilder {
    presenter_id: Option<String>,
    driver_id: Option<String>,
    script: Option<Script>,
    config: Option<Config>,
    created_by: Option<String>,
    presenter_config: Option<PresenterConfig>,
    background: Option<Background>,
    user_data: Option<String>,
    name: Option<String>,
    webhook: Option<String>,
    result_url: Option<String>,
    raw_result_url: Option<String>,
    persist: Option<bool>,
}


impl ClipRequestBodyBuilder {
    pub fn new() -> Self {
        Self {
            presenter_id: None,
            driver_id: None,
            script: None,
            config: None,
            created_by: None,
            presenter_config: None,
            background: None,
            user_data: None,
            name: None,
            webhook: None,
            result_url: None,
            raw_result_url: None,
            persist: None,
        }
    }

    pub fn with_text_script(presenter_id: &str) -> Self {
        let script = Script::Text {
            r#type: "text".to_string(),
            subtitles: false,
            provider: Some(TTSProvider::MicrosoftTTS {
                r#type: "microsoft".to_string(),
                voice_id: "en-US-JennyNeural".to_string(),
            }),
            input: "".to_string(),
            ssml: false,
        };

        Self {
            presenter_id: Some(presenter_id.to_string()),
            driver_id: None,
            script: Some(script),
            config: None,
            created_by: None,
            presenter_config: None,
            background: None,
            user_data: None,
            name: None,
            webhook: None,
            result_url: None,
            raw_result_url: None,
            persist: None,
        }
    }

    pub fn with_audio_script(presenter_id: &str) -> Self {
        let script = Script::Audio {
            r#type: "audio".to_string(),
            subtitles: false,
            audio_url: "".to_string(),
            reduce_noise: false,
        };

        Self {
            presenter_id: Some(presenter_id.to_string()),
            script: Some(script),
            config: None,
            created_by: None,
            presenter_config: None,
            background: None,
            user_data: None,
            name: None,
            webhook: None,
            result_url: None,
            persist: None,
            driver_id: None,
            raw_result_url: None,
        }
    }

    pub fn presenter_id(mut self, presenter_id: &str) -> Self {
        self.presenter_id = Some(presenter_id.to_string());
        self
    }

    pub fn driver_id(mut self, driver_id: &str) -> Self {
        self.driver_id = Some(driver_id.to_string());
        self
    }

    pub fn script(mut self, script: Script) -> Self {
        self.script = Some(script);
        self
    }

    pub fn config(mut self, config: Config) -> Self {
        self.config = Some(config);
        self
    }

    pub fn created_by(mut self, created_by: &str) -> Self {
        self.created_by = Some(created_by.to_string());
        self
    }

    pub fn presenter_config(mut self, presenter_config: PresenterConfig) -> Self {
        self.presenter_config = Some(presenter_config);
        self
    }

    pub fn background(mut self, background: Background) -> Self {
        self.background = Some(background);
        self
    }

    pub fn user_data(mut self, user_data: &str) -> Self {
        self.user_data = Some(user_data.to_string());
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn webhook(mut self, webhook: &str) -> Self {
        self.webhook = Some(webhook.to_string());
        self
    }

    pub fn result_url(mut self, result_url: &str) -> Self {
        self.result_url = Some(result_url.to_string());
        self
    }

    pub fn raw_result_url(mut self, raw_result_url: &str) -> Self {
        self.raw_result_url = Some(raw_result_url.to_string());
        self
    }

    pub fn persist(mut self, persist: bool) -> Self {
        self.persist = Some(persist);
        self
    }

    pub fn audio_url(mut self, audio_url: &str) -> Result<Self> {
        if let Some(Script::Audio { audio_url: a, .. }) = self.script.as_mut() {
            *a = audio_url.to_string();
        }
        Ok(self)
    }

    pub fn input(mut self, input: &str) -> Self {
        if let Some(Script::Text { input: i, .. }) = self.script.as_mut() {
            *i = input.to_string();
        }
        self
    }

    pub fn ssml(mut self, ssml: bool) -> Result<Self> {
        if let Some(Script::Text { ssml: s, .. }) = self.script.as_mut() {
            *s = ssml;
        }
        Ok(self)
    }

    pub fn subtitles(mut self, subtitles: bool) -> Result<Self> {
        if let Some(Script::Text { subtitles: s, .. }) = self.script.as_mut() {
            *s = subtitles;
        }
        Ok(self)
    }

    pub fn provider(mut self, provider: TTSProvider) -> Result<Self> {
        if let Some(Script::Text { provider: p, .. }) = self.script.as_mut() {
            *p = Some(provider);
        }
        Ok(self)
    }

    pub fn reduce_noise(mut self, reduce_noise: bool) -> Result<Self> {
        if let Some(Script::Audio { reduce_noise: r, .. }) = self.script.as_mut() {
            *r = reduce_noise;
        }
        Ok(self)
    }

    pub fn build(self) -> Result<ClipRequestBody> {
        let presenter_id = self.presenter_id.ok_or(RequestBodyBuildError::PresenterIdNotSet)?;
        let script = self.script.ok_or(RequestBodyBuildError::ScriptNotSet)?;

        Ok(ClipRequestBody {
            presenter_id,
            script,
            driver_id: self.driver_id.unwrap_or_default(),
            config: self.config,
            created_by: self.created_by.unwrap_or_default(),
            presenter_config: self.presenter_config,
            background: self.background,
            user_data: self.user_data.unwrap_or_default(),
            name: self.name.unwrap_or_default(),
            webhook: self.webhook.unwrap_or_default(),
            result_url: self.result_url.unwrap_or_default(),
            raw_result_url: self.raw_result_url.unwrap_or_default(),
            persist: self.persist,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateClipResponse {
    id: String,
    object: String,
    created_at: String,
    status: String,
}

pub async fn get_clips() -> Result<GetClipsResponse> {
    let c = ClientBuilder::new()?
        .method(GET)?
        .path(CLIPS_PATH)?
        .header(CONTENT_TYPE, APPLICATION_JSON)?
        .build()?;

    let resp = c.send_request(Empty::<Bytes>::new()).await?;

    let clips = serde_json::from_slice::<GetClipsResponse>(&resp.as_ref())?;

    Ok(clips)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetClipsResponse {
    clips: Vec<Clip>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Clip {
    id: String,
    owner_id: String,
    audio_url: String,
    created_at: String,
    created_by: String,
    modified_at: String,
    started_at: String,
    completed_at: String,
    status: String,
    presenter_id: String,
    driver_id: String,
    config: Config,
    name: String,
    webhook: String,
    result_url: String,
    //metadata: Metadata,
}

pub async fn get_clip(id: &str) -> Result<Clip> {
    let c = ClientBuilder::new()?
        .method(GET)?
        .path(&format!("{}/{}", CLIPS_PATH, id))?
        .header(CONTENT_TYPE, APPLICATION_JSON)?
        .build()?;

    let resp = c.send_request(Empty::<Bytes>::new()).await?;

    let clip = serde_json::from_slice::<Clip>(&resp.as_ref())?;

    Ok(clip)
}

pub async fn delete_clip(id: &str) -> Result<()> {
    let c = ClientBuilder::new()?
        .method(DELETE)?
        .path(&format!("{}/{}", CLIPS_PATH, id))?
        .header(CONTENT_TYPE, APPLICATION_JSON)?
        .build()?;

    let _resp = c.send_request(Empty::<Bytes>::new()).await?;

    Ok(())
}

pub async fn get_presenter_drivers(presenter_id: &str) -> Result<GetPresenterDriversResponse> {
    let c = ClientBuilder::new()?
        .method(GET)?
        .path(&format!("{}{}/{}{}", CLIPS_PATH, PRESENTERS_PATH, presenter_id, DRIVERS_PATH))?
        .header(CONTENT_TYPE, APPLICATION_JSON)?
        .build()?;

    let resp = c.send_request(Empty::<Bytes>::new()).await?;

    let drivers = serde_json::from_slice::<GetPresenterDriversResponse>(&resp.as_ref())?;

    Ok(drivers)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetPresenterDriversResponse {
    clips_drivers: Vec<ClipDriver>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClipDriver {
    created_at: String,
    driver_id: String,
    driver_image_url: String,
    gender: String,
    modified_at: String,
    name: String,
    presenter_id: String,
    preview_url: String,
    thumbnail_url: String,
    video_url: String,
}

