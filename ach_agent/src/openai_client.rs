use reqwest;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Serialize)]
struct OpenAIChatRequest {
    model: String,
    messages: Vec<OpenAIMessage>,
}

#[derive(Serialize, Clone)]
pub struct OpenAIMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct OpenAIChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: Delta,
}

#[derive(Deserialize)]
struct Delta {
    content: String,
}

pub struct OpenAIClient {
    pub api_key: String,
}

impl OpenAIClient {
    pub fn new(api_key: &str) -> Self {
        Self { api_key: api_key.to_string() }
    }
    pub async fn chat(&self, model: &str, history: &[OpenAIMessage]) -> Result<String, reqwest::Error> {
        let req = OpenAIChatRequest {
            model: model.to_string(),
            messages: history.to_vec(),
        };
        let client = reqwest::Client::new();
        let resp = client.post("https://api.openai.com/v1/chat/completions")
            .bearer_auth(&self.api_key)
            .json(&req)
            .timeout(Duration::from_secs(30))
            .send()
            .await?;
        let parsed: OpenAIChatResponse = resp.json().await?;
        Ok(parsed.choices.get(0).map(|c| c.message.content.clone()).unwrap_or_default())
    }
}
