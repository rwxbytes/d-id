pub mod talks;
pub mod clips;

pub use crate::client::*;
pub use crate::prelude::*;
pub use crate::error::*;
pub use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Script {
     Text {
        r#type: String,
        subtitles: bool,
        provider: Option<TTSProvider>,
        input: String,
        ssml: bool,
    },
    Audio {
        r#type: String,
        subtitles: bool,
        #[serde(skip_serializing_if = "String::is_empty")]
        audio_url: String,
        reduce_noise: bool,
    },

}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum TTSProvider {
    MicrosoftTTS {
        r#type: String,
        voice_id: String,
        //voice_config: VoiceConfig,
    },
    ElevenLabsTTS {
        r#type: String,
        voice_id: String,
        //voice_config: VoiceConfig,
    },
    AmazonTTS {
        r#type: String,
        voice_id: String,
        //voice_config: VoiceConfig,
    },
    AfflorithmicsTTS {
        r#type: String,
        voice_id: String,
        //voice_config: VoiceConfig,
    },

}




#[derive(Serialize, Deserialize, Debug)]
pub enum Driver {
    #[serde(rename = "bank://lively/driver-01")]
    LivelyDriver01,
    #[serde(rename = "bank://lively/driver-02")]
    LivelyDriver02,
    #[serde(rename = "bank://lively/driver-03")]
    LivelyDriver03,
    #[serde(rename = "bank://lively/driver-04")]
    LivelyDriver04,
    #[serde(rename = "bank://lively/driver-05")]
    LivelyDriver05,
    #[serde(rename = "bank://lively/driver-06")]
    LivelyDriver06,
    #[serde(rename = "bank://subtle/driver-01")]
    SubtleDriver01,
    #[serde(rename = "bank://subtle/driver-02")]
    SubtleDriver02,
    #[serde(rename = "bank://subtle/driver-03")]
    SubtleDriver03,
    #[serde(rename = "bank://subtle/driver-04")]
    SubtleDriver04,
}



#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    logo: Logo,
    align_driver: bool,
    align_expand_factor: f32,
    auto_match: bool,
    motion_factor: f32,
    normalization_factor: f32,
    sharpen: bool,
    stitch: bool,
    result_format: String,
    fluent: bool,
    pad_audio: f32,
    driver_expression: DriverExpressions,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Logo {
    url: String,
    position: Vec<u32>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DriverExpressions {
    expressions: Vec<ExpressionObject>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExpressionObject {
    start_frame: f32,
    expression: String,
    intensity: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Face {
    size: u32,
    top_left: Vec<u32>,
    overlap: Overlap,
    face_id: String,
    detection_confidence: f32,
    detection: Detection,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum Overlap {
    No,
    Partial,
    Yes,
    Unknown,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Detection {
    top: f32,
    left: f32,
    bottom: f32,
    right: f32,
}
