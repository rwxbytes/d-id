// File: streams.rs
// Path: src/endpoints/video/streams.rs

use super::*;

use webrtc::{api::APIBuilder, peer_connection::configuration::RTCConfiguration, ice_transport::ice_server::*, peer_connection::sdp::session_description::*};

const STREAMS_PATH: &str = "/talks/streams";
const SDP_PATH: &str = "/sdp";
const ICE_PATH: &str = "/ice";

#[derive(Serialize, Debug)]
pub struct NewStreamRequestBody {
    pub source_url: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub driver_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face: Option<Face>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<StreamConfig>,
}

impl NewStreamRequestBody {
    pub async fn create_stream(&self) -> Result<NewStreamResponse> {
        let c = ClientBuilder::new()?
            .method(POST)?
            .path(STREAMS_PATH)?
            .header(ACCEPT, APPLICATION_JSON)?
            .header(CONTENT_TYPE, APPLICATION_JSON)?
            .build()?;

        let body = serde_json::to_string(&self)?;

        let resp = c.send_request(Full::<Bytes>::new(body.into())).await?;

        let stream_resp = serde_json::from_slice::<NewStreamResponse>(&resp.as_ref())?;

        Ok(stream_resp)
    }
}


#[derive(Serialize, Debug)]
pub struct NewStreamRequestBodyBuilder {
    pub source_url: Option<String>,
    pub driver_url: Option<String>,
    pub face: Option<Face>,
    pub config: Option<StreamConfig>,
}

#[derive(Serialize, Debug)]
pub struct StreamConfig {
    pub motion_factor: f64,
    pub align_expand_factor: f64,
    pub stitch: bool,
}

impl NewStreamRequestBodyBuilder {
    pub fn new() -> Self {
        Self {
            source_url: None,
            driver_url: None,
            face: None,
            config: None,
        }
    }

    pub fn source_url(mut self, source_url: &str) -> Self {
        self.source_url = Some(source_url.to_string());
        self
    }

    pub fn driver_url(mut self, driver_url: &str) -> Self {
        self.driver_url = Some(driver_url.to_string());
        self
    }

    pub fn face(mut self, face: Face) -> Self {
        self.face = Some(face);
        self
    }

    pub fn config(mut self, config: StreamConfig) -> Self {
        self.config = Some(config);
        self
    }

    pub fn build(self) -> Result<NewStreamRequestBody> {
        let source_url = self.source_url.ok_or(RequestBodyBuildError::SourceUrlNotSet)?;

        Ok(NewStreamRequestBody {
            source_url: source_url,
            driver_url: self.driver_url.unwrap_or_default(),
            face: self.face,
            config: self.config,
        })
    }
}


#[derive(Deserialize, Debug)]
pub struct NewStreamResponse {
    id: String,
    offer: Offer,
    ice_servers: Vec<IceServer>,
    session_id: String,
}

#[derive(Deserialize, Debug)]
pub struct IceServer {
    urls: Urls,
    username: Option<String>,
    credential: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Urls {
    Stun(String),
    Turns(Vec<String>),
}

#[derive(Deserialize, Debug)]
pub struct Offer {
    r#type: String,
    sdp: String,
}

pub async fn start_stream(stream_response: NewStreamResponse) -> Result<serde_json::Value> {
    let web_rtc_api = APIBuilder::new().build();

    let ice_servers = stream_response.ice_servers.iter().map(|ice_server| {
        let urls = match &ice_server.urls {
            Urls::Stun(url) => vec![url.clone()],
            Urls::Turns(urls) => urls.clone(),
        };

        RTCIceServer {
            urls,
            //username: ice_server.username.clone().unwrap_or_default(),
            credential: ice_server.credential.clone().unwrap_or_default(),
            //credential_type: RTCIceCredentialType::Password,
            ..Default::default()
        }
    }).collect::<Vec<_>>();

    let rtc_config = RTCConfiguration {
        ice_servers,
        peer_identity: stream_response.id.clone(),
        ..Default::default()
    };

    let peer_connection = web_rtc_api.new_peer_connection(rtc_config).await?;

    let rtc_session_offer = RTCSessionDescription::offer(stream_response.offer.sdp)?;

    let _ = peer_connection.set_remote_description(rtc_session_offer).await?;

    let rtc_session_answer = peer_connection.create_answer(None).await?;


    let _ = peer_connection.set_local_description(rtc_session_answer.clone()).await?;

    //dbg!(peer_connection.connection_state());

    let c = ClientBuilder::new()?
        .method(POST)?
        .path(&format!("{}/{}{}", STREAMS_PATH, stream_response.id, SDP_PATH))?
        .header(ACCEPT, APPLICATION_JSON)?
        .header(CONTENT_TYPE, APPLICATION_JSON)?
        .build()?;

    let body = StartStreamRequestBody::new(rtc_session_answer.sdp, stream_response.session_id);

    let body = serde_json::to_string(&body)?;

    let resp = c.send_request(Full::<Bytes>::new(body.into())).await?;

    let json = serde_json::from_slice::<serde_json::Value>(&resp.as_ref())?;

    Ok(json)

}

#[derive(Serialize, Debug)]
pub struct StartStreamRequestBody {
    pub answer: Answer,
    pub session_id: String,
}

#[derive(Serialize, Debug)]
pub struct Answer {
    pub r#type: String,
    pub sdp: String,
}

impl StartStreamRequestBody {
    pub fn new(sdp: String, session_id: String) -> Self {
        Self {
            answer: Answer {
                r#type: "answer".to_string(),
                sdp: sdp,
            },
            session_id: session_id,
        }
    }
}

