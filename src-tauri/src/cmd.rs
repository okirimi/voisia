use tauri::command;
use reqwest;

use crate::api::{load_anthropic_api_key, load_openai_api_key};
use crate::dto::{
    AnthropicRequest,
    AnthropicResponse,
    AnthropicSystemMessage,
    AnthropicThinking,
    OpenAIRequest,
    OpenAIResponse,
    VoisiaMessage,
};
use crate::llm::{load_json, ModelInfo};

async fn call_anthropic_api(
    model: String,
    input: Vec<VoisiaMessage>,
    system: Option<String>,
    max_tokens: u32,
    temperature: f32,
    top_p: f32,
    thinking: Option<AnthropicThinking>,
) -> Result<AnthropicResponse, String> {
    let api_key = load_anthropic_api_key()
        .map_err(|e| format!("Failed to load API key: {}", e))?;

    let http_client = reqwest::Client::new();

    // Build request body for Anthropic API
    let system_messages = system.map(|s| vec![AnthropicSystemMessage {
        r#type: "text".to_string(),
        text: s,
    }]);

    let request_body = AnthropicRequest {
        model,
        messages: input,
        system: system_messages,
        max_tokens,
        temperature,
        top_p,
        thinking,
    };

    log::info!("Anthropic API request body: {:?}", request_body);

    // Send POST request to Anthropic API
    let response = http_client
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", &api_key)
        .header("anthropic-version", "2023-06-01")
        .header("content-type", "application/json")
        .json(&request_body)
        .send()
        .await
        .map_err(|e| format!("Anthropic HTTP request failed: {}", e))?;

        // Check response status code
        if response.status().is_success() {
            let anthropic_response: AnthropicResponse = response
                .json()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))?;

            log::info!("**SUCCESS** - Anthropic API response received: {:?}", anthropic_response);
            Ok(anthropic_response)
        } else {
            let status = response.status();
            let err_txt = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            let err_msg = format!("API call failed with status {}: {}", status, err_txt);
            log::error!("**ERROR** - {}", err_msg);
            Err(err_msg)
        }
}

async fn call_openai_api(
    input: Vec<VoisiaMessage>,
    model: String,
    max_tokens: u32,
    store: bool,
    system: Option<String>,
    temperature: f32,
    top_p: f32
) -> Result<OpenAIResponse, String> {
    let api_key = load_openai_api_key()
        .map_err(|e| format!("Failed to load OpenAI API key: {}", e))?;

    let http_client = reqwest::Client::new();

    // Build request body for OpenAI Responses API
    let request_body = OpenAIRequest {
        input: input,  // Changed from 'messages' to 'input'
        model,
        max_output_tokens: max_tokens,
        store,
        instructions: system,  // Changed from 'instruction' to 'instructions'
        temperature,
        top_p,
    };

    let response = http_client
        .post("https://api.openai.com/v1/responses")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await
        .map_err(|e| format!("OpenAI HTTP request failed: {}", e))?;

    // Check response status code
    if response.status().is_success() {
        let openai_response: OpenAIResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse OpenAI response: {}", e))?;

        log::info!("**SUCCESS** - OpenAI API response received: {:?}", openai_response);
        Ok(openai_response)
    } else {
        let status = response.status();
        let err_txt = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        let err_msg = format!("OpenAI API call failed with status {}: {}", status, err_txt);
        log::error!("**ERROR** - {}", err_msg);
        Err(err_msg)
    }
}

#[command]
pub async fn generate_anthropic_response(
    model: String,
    input: String,
    system: Option<String>,
    max_tokens: u32,
    temperature: f32,
    top_p: f32,
    thinking: Option<AnthropicThinking>,
    convo_history: Vec<VoisiaMessage>,
) -> Result<AnthropicResponse, String> {
    // Convert existing conversation history to Anthropic API format
    let mut messages: Vec<VoisiaMessage> = convo_history
        .into_iter()
        .map(|msg| VoisiaMessage {
            role: msg.role,
            content: msg.content,
        })
        .collect();

    // Add new message to the conversation history
    messages.push(VoisiaMessage {
        role: "user".to_string(),
        content: input,
    });

    // Call Anthropic API and get response
    let response = call_anthropic_api(
        model,
        messages,
        system,
        max_tokens,
        temperature,
        top_p,
        thinking,
    ).await?;

    Ok(response)
}

#[command]
pub async fn generate_openai_response(
    model: String,
    input: String,
    max_tokens: u32,
    temperature: f32,
    top_p: f32,
    store: bool,
    system: Option<String>,
    conversation_history: Vec<VoisiaMessage>,
) -> Result<OpenAIResponse, String> {
    // Convert existing conversation history
    let mut messages: Vec<VoisiaMessage> = conversation_history
        .into_iter()
        .map(|msg| VoisiaMessage {
            role: msg.role,
            content: msg.content,
        })
        .collect();

    // Add new message to the conversation history
    messages.push(VoisiaMessage {
        role: "user".to_string(),
        content: input,
    });

    // Call OpenAI API and get response
    let response = call_openai_api(
        messages,
        model,
        max_tokens,
        store,
        system,
        temperature,
        top_p,
    ).await?;

    Ok(response)
}

#[command]
pub fn get_available_models() -> Result<Vec<ModelInfo>, String> {
    let models_path = "resources/llm-info.json";

    match load_json(models_path) {
        Ok(root) => Ok(root.models),
        Err(e) => {
            log::error!("Failed to load models from {}: {}", models_path, e);
            Err(format!("Failed to load models: {}", e))
        }
    }
}
