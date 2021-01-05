use super::*;

/// History
#[derive(Serialize, Deserialize)]
struct Meta {
    #[serde(alias = "topic")]
    topic: Vec<Topic>,
}

#[derive(Serialize, Deserialize)]
struct Topic {
    name: String,
    paused: bool,
    channels: Vec<Channels>,
}

#[derive(Serialize, Deserialize)]
struct Channels {
    name: String,
    paused: bool,
}