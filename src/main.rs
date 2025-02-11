use axum::{
    routing::post,
    Router,
    Json,
    extract::State,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

mod chat;
mod functions;
mod history;

use chat::{handle_chat, ChatRequest};

#[derive(Clone)]
pub struct AppState {
    kv_store: Arc<Mutex<history::KVStore>>,
    groq_api_key: String,
    openweathermap_api_key: String,
}

#[tokio::main]
async fn main() {
    let app_state = AppState {
        kv_store: Arc::new(Mutex::new(history::KVStore::new())),
        groq_api_key: std::env::var("GROQ_API_KEY").expect("GROQ_API_KEY must be set"),
        openweathermap_api_key: std::env::var("OPENWEATHERMAP_API_KEY")
            .expect("OPENWEATHERMAP_API_KEY must be set"),
    };

    let app = Router::new()
        .route("/", post(handle_request))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Server running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn handle_request(
    State(state): State<AppState>,
    Json(body): Json<ChatRequest>,
) -> Json<serde_json::Value> {
    match handle_chat(state, body).await {
        Ok(response) => Json(serde_json::json!({ "response": response })),
        Err(e) => Json(serde_json::json!({
            "response": "Something went wrong, we are working on it",
            "error": e.to_string()
        })),
    }
} 