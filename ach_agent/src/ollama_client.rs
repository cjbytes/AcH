use reqwest;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Serialize)]
struct OllamaChatRequest {
    model: String,
    messages: Vec<OllamaMessage>,
}

pub struct OllamaMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct OllamaChatResponse {
    message: String,
}

pub struct OllamaClient {
    pub base_url: String,
}

impl OllamaClient {
    pub fn new(base_url: &str) -> Self {
        Self { base_url: base_url.to_string() }
    }
    pub async fn chat(&self, model: &str, history: &[OllamaMessage]) -> Result<String, reqwest::Error> {
        let req = OllamaChatRequest {
            model: model.to_string(),
            messages: history.to_vec(),
        };
        let client = reqwest::Client::new();
        let resp = client.post(format!("{}/api/chat", self.base_url))
            .json(&req)
            .timeout(Duration::from_secs(30))
            .send()
            .await?;
        let parsed: OllamaChatResponse = resp.json().await?;
        Ok(parsed.message)
    }
}

eframe = { version = "0.24", features = ["wgpu", "glow", "persistence", "default_fonts", "windows"] }
