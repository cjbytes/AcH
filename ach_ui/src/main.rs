use eframe::{egui, Theme};
use ach_agent::AiChatAgent;
use std::sync::Arc;
use tokio::runtime::Runtime;

struct AICHTApp {
    agent: AiChatAgent,
    input: String,
    messages: Vec<String>,
    rt: Arc<Runtime>,
}

impl Default for AICHTApp {
    fn default() -> Self {
        let rt = Arc::new(Runtime::new().unwrap());
        Self {
            agent: AiChatAgent::new(),
            input: String::new(),
            messages: vec![],
            rt,
        }
    }
}

impl eframe::App for AICHTApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("AICHAT: Rust Native AI Agent");
            for msg in &self.messages {
                ui.label(msg);
            }
            ui.add(egui::TextEdit::singleline(&mut self.input).desired_width(f32::INFINITY));
            if ui.button("Send").clicked() {
                let user_msg = self.input.clone();
                self.messages.push(format!("You: {}", user_msg));
                self.input.clear();
                // Use backend agent for response
                let agent = self.agent.clone();
                let rt = self.rt.clone();
                let response = rt.block_on(async move {
                    agent.add_message("user", &user_msg).await;
                    // For now, simulate response
                    let reply = "Agent: Hello! I'm your Rust-powered AI. Try asking me to explain code or write a function!".to_string();
                    agent.add_message("agent", &reply).await;
                    reply
                });
                self.messages.push(response);
            }
        });
    }
}

fn main() {
    println!("ACH agent started");
    // Optionally call into your library here
    // ach_agent::run();
}
