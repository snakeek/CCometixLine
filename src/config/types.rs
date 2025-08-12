use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub theme: String,
    pub segments: SegmentsConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SegmentsConfig {
    pub directory: bool,
    pub git: bool,
    pub model: bool,
    pub time: bool,
    pub usage: bool,
    pub cost: bool,
}

// Data structures compatible with existing main.rs
#[derive(Deserialize)]
pub struct Model {
    pub display_name: String,
}

#[derive(Deserialize)]
pub struct Workspace {
    pub current_dir: String,
}

#[derive(Deserialize)]
pub struct InputData {
    pub model: Model,
    pub workspace: Workspace,
    pub transcript_path: String,
}

#[derive(Deserialize)]
pub struct Usage {
    pub input_tokens: u32,
    pub output_tokens: u32,
    pub cache_read_input_tokens: u32,
    // 智谱清言可能没有这个字段，设为可选
    #[serde(default)]
    pub cache_creation_input_tokens: u32,
    // GLM模型可能有的字段
    #[serde(default)]
    pub total_tokens: u32,
}

#[derive(Deserialize)]
pub struct MessageContent {
    #[serde(rename = "type")]
    pub content_type: String,
    pub text: Option<String>,
}

#[derive(Deserialize)]
pub struct Message {
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub message_type: String,
    pub role: String,
    pub model: Option<String>,
    pub content: Option<Vec<MessageContent>>,
    pub usage: Option<Usage>,
}

#[derive(Deserialize)]
pub struct TranscriptEntry {
    #[serde(rename = "type")]
    pub entry_type: String,
    pub message: Option<Message>,
    // 智谱清言特有的字段
    pub cwd: Option<String>,
    #[serde(rename = "gitBranch")]
    pub git_branch: Option<String>,
    pub uuid: Option<String>,
    pub timestamp: Option<String>,
}