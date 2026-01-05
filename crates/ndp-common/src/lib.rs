use serde::{Deserialize, Serialize};

pub const DEFAULT_PORT: u16 = 5510;
pub const DEFAULT_CONTROL_PORT: u16 = 5511;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "payload")]
pub enum ControlMessage {
    #[serde(rename = "HELLO")]
    Hello(Hello),
    #[serde(rename = "WELCOME")]
    Welcome(Welcome),
    #[serde(rename = "PING")]
    Ping { timestamp: u64 },
    #[serde(rename = "PONG")]
    Pong { timestamp: u64 },
    #[serde(rename = "BYE")]
    Bye { reason: String },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Hello {
    pub version: String,
    pub supported_codecs: Vec<String>,
    pub max_packet_size: u32,
    pub token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Welcome {
    pub session_id: String,
    pub codec: String,
    pub port: u16, // RTP port
}