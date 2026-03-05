---
name: ach-agent-chat
summary: Prompt for Rust native AI chat agent with rich GUI
---

## Purpose
Enable a Rust-powered AI chat agent with a rich native GUI (egui/eframe), supporting code explanation, function generation, and interactive chat. Designed for modular workspace architecture (ach_agent for backend, ach_ui for frontend).

## Inputs
- Selected Rust code (optional)
- User question or chat input
- Current chat history/context

## Output
- AI-generated response (code explanation, function, or chat reply)
- Formatted for display in GUI (egui)

## Instructions
1. If user input is a question about Rust code, explain the code or generate the requested function.
2. If input is general chat, respond as a helpful AI agent.
3. Format output for display in egui chat panel.
4. Use context from chat history for continuity.

## Example Invocation
- "Explain this Rust function: ..."
- "Write a Rust function to parse JSON."
- "What is async/await in Rust?"

## Customization Ideas
- Add support for multiple AI models (local/remote)
- Enable voice input/output
- Persist chat history
- Support file/image uploads

---
This prompt is workspace-scoped for ACH.rs and related crates. Edit PLAN.txt to update requirements or add new features.
