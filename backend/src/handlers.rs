use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};

use crate::filters;

#[derive(Deserialize)]
pub struct ChatRequest {
    pub message: String,
}

#[derive(Serialize)]
pub struct ChatResponse {
    pub reply: String,
    pub blocked: bool,
    pub reason: Option<String>,
}

pub async fn chat_handler(Json(payload): Json<ChatRequest>) -> impl IntoResponse {
    let message = payload.message.trim();
    if message.is_empty() {
        let res = ChatResponse {
            reply: "empty message".into(),
            blocked: true,
            reason: Some("empty".into()),
        };
        return (StatusCode::BAD_REQUEST, Json(res));
    }

    if let Some(patt) = filters::violates_blocklist(message) {
        let res = ChatResponse {
            reply: "This assistant only supports safe educational questions.".into(),
            blocked: true,
            reason: Some(format!("policy_block: {}", patt)),
        };
        return (StatusCode::FORBIDDEN, Json(res));
    }

    if let Some(patt) = filters::asks_for_dangerous_instructions(message) {
        let res = ChatResponse {
            reply: "I can't provide instructions for harmful or illegal activities.".into(),
            blocked: true,
            reason: Some(format!("dangerous_instruction: {}", patt)),
        };
        return (StatusCode::FORBIDDEN, Json(res));
    }

    if !filters::is_educational_intent(message) {
        let res = ChatResponse {
            reply: "Please reframe your question to an educational topic (explain, example, practice).".into(),
            blocked: true,
            reason: Some("not_educational".into()),
        };
        return (StatusCode::FORBIDDEN, Json(res));
    }

    let reply = if message.to_lowercase().contains("hello") {
        "Hello! I'm Guidance — your educational assistant.".to_string()
    } else {
        format!(
            "Let's break that down:\n\n1) Quick definition.\n2) Example.\n3) Practice.\n\nYou asked: {}",
            message
        )
    };

    let res = ChatResponse {
        reply,
        blocked: false,
        reason: None,
    };
    (StatusCode::OK, Json(res))
}
