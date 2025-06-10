use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{Error, ErrorKind};

#[derive(Debug, Deserialize, Serialize)]
pub struct ModelInfo {
    // Unique identifier for the model
    pub id: String,
    // Display name for the model
    pub display_name: String,
    // Provider of the model
    pub provider: String,
    // Optional tags to classify the model
    pub tags: Vec<String>,
    // Parameters to control the model's behavior
    pub params: ModelParams,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ModelParams {
    pub max_tokens: u32,
    pub temperature: f32,
    pub top_p: f32,
}

#[derive(Debug,Deserialize, Serialize)]
pub struct Root {
    pub models: Vec<ModelInfo>,
}

impl Default for ModelInfo {
    fn default() -> Self {
        Self {
            id: String::new(), // A mutable string stored on the heap
            display_name: String::new(),
            provider: String::new(),
            tags: Vec::new(),
            params: ModelParams::default(),
        }
    }
}

pub fn load_json(path: &str) -> Result<Root, Error> {
    let json_str = fs::read_to_string(path)?;
    serde_json::from_str(&json_str).map_err(|e| {
        let msg = format!("Failed to parse JSON file {}: {}", path, e);
        Error::new(ErrorKind::InvalidData, msg)
    })
}
