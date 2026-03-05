pub mod config;
pub mod storage;
pub mod ollama_client;
pub mod openai_client;

use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub sender: String,
    pub content: String,
}

#[derive(Debug)]
pub struct AiChatAgent {
    pub history: Arc<Mutex<Vec<ChatMessage>>>,
}

impl AiChatAgent {
    pub fn new() -> Self {
        Self {
            history: Arc::new(Mutex::new(vec![])),
        }
    }
    pub async fn add_message(&self, sender: &str, content: &str) {
        let mut history = self.history.lock().await;
        history.push(ChatMessage {
            sender: sender.to_string(),
            content: content.to_string(),
        });
    }
    pub async fn get_history(&self) -> Vec<ChatMessage> {
        let history = self.history.lock().await;
        history.clone()
    }
}
