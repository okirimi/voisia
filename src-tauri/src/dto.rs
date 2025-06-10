use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AnthropicContent {
    pub text: String,
    pub r#type: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum AnthropicContentWithThinking {
    #[serde(rename = "thinking")]
    Thinking {
        thinking: String,
        signature: String,
    },
    #[serde(rename = "text")]
    Text {
        text: String,
    },
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AnthropicRequest {
    pub messages: Vec<VoisiaMessage>,
    pub model: String,
    pub max_tokens: u32,  // Required range: x >= 1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<Vec<AnthropicSystemMessage>>,
    pub temperature: f32, // Required range: 0 <= x <= 1
    pub top_p: f32,       // Required range: 0 <= x <= 1
    // This field will not be serialized if it is None
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thinking: Option<AnthropicThinking>,
}

impl AnthropicRequest {
    pub fn validate(&self) -> Result<(), String> {
        if self.max_tokens < 1 {
            return Err("max_tokens must be greater than 1".to_string());
        }
        if self.temperature < 0.0 || self.temperature > 1.0 {
            return Err("Temperature must be between 0 and 1".to_string());
        }
        if self.top_p < 0.0 || self.top_p > 1.0 {
            return Err("top_p must be between 0 and 1".to_string());
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AnthropicResponse {
    pub content: Vec<AnthropicContent>,
    pub id: String,
    pub model: String,
    pub role: String,
    pub stop_reason: String,
    pub stop_sequence: Option<String>,
    pub r#type: String,
    pub usage: AnthropicUsage,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AnthropicSystemMessage {
    pub r#type: String,
    pub text: String,
}

fn default_thinking_type() -> String {
    "disabled".to_string()
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct AnthropicThinking {
    #[serde(default = "default_thinking_type")]
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_tokens: Option<u32>, // Must be ≥1024 and less than max_tokens
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AnthropicUsage {
    pub input_tokens: u32,
    pub output_tokens: u32,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct OpenAIFormatType {
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct OpenAIMetadata {}

#[derive(Debug, Deserialize, Serialize)]
pub struct OpenAIOutputContent {
    pub r#type: String,
    pub text: String,
    pub annotations: Vec<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OpenAIOutputMessage {
    pub r#type: String,
    pub id: String,
    pub status: String,
    pub role: String,
    pub content: Vec<OpenAIOutputContent>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OpenAIRequest {
    pub input: Vec<VoisiaMessage>,
    pub model: String,
    pub max_output_tokens: u32,
    pub store: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    pub temperature: f32,
    pub top_p: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OpenAIResponse {
    pub id: String,
    pub object: String,
    pub created_at: u64,
    pub status: String,
    pub error: Option<serde_json::Value>,
    pub incomplete_details: Option<serde_json::Value>,
    pub instructions: Option<serde_json::Value>,
    pub max_output_tokens: Option<u32>,
    pub model: String,
    pub output: Vec<OpenAIOutputMessage>,
    pub parallel_tool_calls: bool,
    pub previous_response_id: Option<String>,
    // o-series はここに reasoning が必要
    pub store: bool,
    pub temperature: f32,
    // text (Configuration options for a text response from the model.) 必要に応じて
    pub tool_choice: String,
    pub tools: Vec<serde_json::Value>,
    pub top_p: f32,
    pub truncation: String,
    pub usage: Option<OpenAIUsage>,
    pub user: Option<String>,
    pub metadata: Option<OpenAIMetadata>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct OpenAITextFormat {
    pub format: Option<OpenAIFormatType>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct OpenAITokenDetails {
    pub cached_tokens: Option<u32>,
    pub reasoning_tokens: Option<u32>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct OpenAIUsage {
    pub input_tokens: u32,
    pub input_tokens_details: Option<OpenAITokenDetails>,
    pub output_tokens: u32,
    pub output_tokens_details: Option<OpenAITokenDetails>,
    pub total_tokens: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VoisiaMessage {
    pub role: String,
    pub content: String,
}
