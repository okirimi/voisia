use std::{env, io::{Error, ErrorKind}};

pub fn load_anthropic_api_key() -> Result<String, Error> {
    env::var("ANTHROPIC_API_KEY").map_err(|_| {
        Error::new(ErrorKind::NotFound, "Anthropic API key is not found")
    })
}

pub fn load_gemini_api_key() -> Result<String, Error> {
    env::var("GEMINI_API_KEY").map_err(|_| {
        Error::new(ErrorKind::NotFound, "Gemini API key is not found")
    })
}

pub fn load_openai_api_key() -> Result<String, Error> {
    env::var("OPENAI_API_KEY").map_err(|_| {
        Error::new(ErrorKind::NotFound, "OpenAI API key is not found")
    })
}

pub fn load_anthropic_endpoint() -> Result<String, Error> {
    env::var("ANTHROPIC_API_ENDPOINT").map_err(|_| {
        Error::new(ErrorKind::NotFound, "Anthropic API endpoint is not found")
    })
}

pub fn load_openai_endpoint() -> Result<String, Error> {
    env::var("OPENAI_API_ENDPOINT").map_err(|_| {
        Error::new(ErrorKind::NotFound, "OpenAI API endpoint is not found")
    })
}
