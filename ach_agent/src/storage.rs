use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use dirs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedChatHistory {
    pub messages: Vec<ChatMessage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub sender: String,
    pub content: String,
}

pub async fn save_history(history: &[ChatMessage]) -> Result<(), Box<dyn std::error::Error>> {
    let path = history_path();
    let data = serde_json::to_string(&SavedChatHistory { messages: history.to_vec() })?;
    fs::write(path, data)?;
    Ok(())
}

pub async fn load_history() -> Result<Vec<ChatMessage>, Box<dyn std::error::Error>> {
    let path = history_path();
    if path.exists() {
        let data = fs::read_to_string(path)?;
        let history: SavedChatHistory = serde_json::from_str(&data)?;
        Ok(history.messages)
    } else {
        Ok(vec![])
    }
}

fn history_path() -> PathBuf {
    dirs::config_dir().unwrap_or_else(|| PathBuf::from(".")).join("aichat/chat_history.json")
}
