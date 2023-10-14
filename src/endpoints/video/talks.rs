// File: talks.rs
// Path: src/endpoints/video/talks.rs


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn talk_request_body_is_formatting_with_text_script() {
        let talk_req_bod = TalkRequestBody {
            source_url: "www.dummyurl.com".to_string(),
            driver_url: None,
            script: Script::Text {
                r#type: "text".to_string(),
                subtitles: false,
                provider: Some(TTSProvider::MicrosoftTTS {
                    r#type: "microsoft".to_string(),
                    voice_id: "en-US-JennyNeural".to_string(),
                }),
                input: "Hello world!".to_string(),
                ssml: false,
            },
            config: None,
            user_data: "".to_string(),
            name: "".to_string(),
            webhook: "".to_string(),
            result_url: "".to_string(),
            face: None,
            persist: false,
        };

        let talk_req_bod2 = TalkRequestBodyBuilder::with_text_script().unwrap()
            .source_url("www.dummyurl.com").unwrap()
            .input("Hello world!").unwrap()
            .build().unwrap();

        let want = serde_json::to_string(&talk_req_bod).unwrap();
        let got = serde_json::to_string(&talk_req_bod2).unwrap();

        assert_eq!(want, got);


    }

    #[test]
    fn talk_request_body_is_formatting_with_audio_script() {
        let talk_req_bod = TalkRequestBody {
            source_url: "www.dummyurl.com".to_string(),
            driver_url: None,
            script: Script::Audio {
                r#type: "audio".to_string(),
                subtitles: false,
                audio_url: "www.dummyaudiourl.com".to_string(),
                reduce_noise: false,
            },
            config: None,
            user_data: "".to_string(),
            name: "".to_string(),
            webhook: "".to_string(),
            result_url: "".to_string(),
            face: None,
            persist: false,
        };

        let talk_req_bod2 = TalkRequestBodyBuilder::with_audio_script().unwrap()
            .source_url("www.dummyurl.com").unwrap()
            .audio_url("www.dummyaudiourl.com").unwrap()
            .build().unwrap();

        let want = serde_json::to_string(&talk_req_bod).unwrap();
        let got = serde_json::to_string(&talk_req_bod2).unwrap();

        assert_eq!(want, got);
    }
}


use super::*;


const TALKS_PATH: &str = "/talks";

#[derive(Serialize, Deserialize, Debug)]
pub struct TalkRequestBody {
    source_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    driver_url: Option<Driver>,
    script: Script,
    #[serde(skip_serializing_if = "Option::is_none")]
    config: Option<Config>,
    #[serde(skip_serializing_if = "String::is_empty")]
    user_data: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    webhook: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    result_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    face: Option<Face>,
    persist: bool,
}

impl TalkRequestBody {
    pub async fn create_talk(&self) -> Result<CreateTalkResponse> {
        let c = ClientBuilder::new()?
            .method(POST)?
            .path(TALKS_PATH)?
            .header(CONTENT_TYPE, APPLICATION_JSON)?
            .build()?;

        let resp = c.send_request(Full::<Bytes>::new(serde_json::to_string(&self)?.into())).await?;

        let json = serde_json::from_slice::<CreateTalkResponse>(&resp.as_ref())?;

        Ok(json)
    }


}
pub async fn get_talk(id: &str) -> Result<GetTalkResponse> {
    let c = ClientBuilder::new()?
        .method(GET)?
        .path(&format!("{}/{}", TALKS_PATH, id))?
        .header(CONTENT_TYPE, APPLICATION_JSON)?
        .build()?;

    let resp = c.send_request(Empty::<Bytes>::new()).await?;

    let json = serde_json::from_slice::<GetTalkResponse>(&resp.as_ref())?;

    Ok(json)
}

pub async fn get_talks() -> Result<GetTalksResponse> {
    let c = ClientBuilder::new()?
        .method(GET)?
        .path(TALKS_PATH)?
        .header(CONTENT_TYPE, APPLICATION_JSON)?
        .build()?;

    let resp = c.send_request(Empty::<Bytes>::new()).await?;

    let json = serde_json::from_slice::<GetTalksResponse>(&resp.as_ref())?;

    Ok(json)
}

pub async fn delete_talk(id: &str) -> Result<()> {
    let c = ClientBuilder::new()?
        .method(DELETE)?
        .path(&format!("{}/{}", TALKS_PATH, id))?
        .header(CONTENT_TYPE, APPLICATION_JSON)?
        .build()?;

    let _resp = c.send_request(Empty::<Bytes>::new()).await?;

    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateTalkResponse {
    pub id: String,
    pub object: String,
    pub created_by: String,
    pub created_at: String,
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetTalkResponse {
    pub id: String,
    pub user_id: String,
    pub source_url: String,
    pub created_at: String,
    pub audio_url: String,
    pub started_at: String,
    pub modified_at: String,
    pub status: String,
    pub result_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetTalksResponse {
    pub talks: Vec<GetTalkResponse>,
}



pub struct TalkRequestBodyBuilder {
    source_url: Option<String>,
    driver_url: Option<Driver>,
    script: Option<Script>,
    config: Option<Config>,
    user_data: Option<String>,
    name: Option<String>,
    webhook: Option<String>,
    result_url: Option<String>,
    face: Option<Face>,
    persist: Option<bool>,
}

impl TalkRequestBodyBuilder {
    pub fn with_text_script() -> Result<Self> {
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

        Ok(Self {
            source_url: None,
            driver_url: None,
            script: Some(script),
            config: None,
            user_data: None,
            name: None,
            webhook: None,
            result_url: None,
            face: None,
            persist: None,
        })
    }

    pub fn with_audio_script() -> Result<Self> {
        let script = Script::Audio {
            r#type: "audio".to_string(),
            subtitles: false,
            audio_url: "".to_string(),
            reduce_noise: false,
        };

        Ok(Self {
            source_url: None,
            driver_url: None,
            script: Some(script),
            config: None,
            user_data: None,
            name: None,
            webhook: None,
            result_url: None,
            face: None,
            persist: None,
        })
    }
    pub fn source_url(mut self, source_url: &str) -> Result<Self> {
        self.source_url = Some(source_url.to_string());
        Ok(self)
    }

    fn audio_url(mut self, audio_url: &str) -> Result<Self> {
        if let Some(Script::Audio { audio_url: a, .. }) = self.script.as_mut() {
            *a = audio_url.to_string();
        }
        Ok(self)
    }

    fn driver_url(mut self, driver_url: Driver) -> Result<Self> {
        self.driver_url = Some(driver_url);
        Ok(self)
    }

    fn script(mut self, script: Script) -> Result<Self> {
        self.script = Some(script);
        Ok(self)
    }

    fn user_data(mut self, user_data: &str) -> Result<Self> {
        self.user_data = Some(user_data.to_string());
        Ok(self)
    }

    fn name(mut self, name: &str) -> Result<Self> {
        self.name = Some(name.to_string());
        Ok(self)
    }

    fn webhook(mut self, webhook: &str) -> Result<Self> {
        self.webhook = Some(webhook.to_string());
        Ok(self)
    }

    fn result_url(mut self, result_url: &str) -> Result<Self> {
        self.result_url = Some(result_url.to_string());
        Ok(self)
    }

    fn persist(mut self, persist: bool) -> Result<Self> {
        self.persist = Some(persist);
        Ok(self)
    }

    pub fn input (mut self, input: &str) -> Result<Self> {
        if let Some(Script::Text { input: i, .. }) = self.script.as_mut() {
            *i = input.to_string();
        }
        Ok(self)
    }

    fn ssml(mut self, ssml: bool) -> Result<Self> {
        if let Some(Script::Text { ssml: s, .. }) = self.script.as_mut() {
            *s = ssml;
        }
        Ok(self)
    }

    fn subtitles(mut self, subtitles: bool) -> Result<Self> {
        if let Some(Script::Text { subtitles: s, .. }) = self.script.as_mut() {
            *s = subtitles;
        }
        Ok(self)
    }

    fn provider(mut self, provider: TTSProvider) -> Result<Self> {
        if let Some(Script::Text { provider: p, .. }) = self.script.as_mut() {
            *p = Some(provider);
        }
        Ok(self)
    }

    fn face(mut self, face: Face) -> Result<Self> {
        self.face = Some(face);
        Ok(self)
    }

    fn config(mut self, config: Config) -> Result<Self> {
        self.config = Some(config);
        Ok(self)
    }

    fn reduce_noise(mut self, reduce_noise: bool) -> Result<Self> {
        if let Some(Script::Audio { reduce_noise: r, .. }) = self.script.as_mut() {
            *r = reduce_noise;
        }
        Ok(self)
    }



    pub fn build(self) -> Result<TalkRequestBody> {
        let source_url = self.source_url.ok_or(RequestBodyBuildError::SourceUrlNotSet)?;

        let script = self.script.ok_or(Box::new(RequestBodyBuildError::ScriptNotSet))?;

        Ok(
            TalkRequestBody {
                source_url,
                driver_url: self.driver_url,
                script,
                config: self.config,
                user_data: self.user_data.unwrap_or("".to_string()),
                name: self.name.unwrap_or("".to_string()),
                webhook: self.webhook.unwrap_or("".to_string()),
                result_url: self.result_url.unwrap_or("".to_string()),
                face: self.face,
                persist: self.persist.unwrap_or(false),
            }
        )

    }


}

