use axum::{
    routing::{get, post},
    Json, Router,
};
use serde_json::{json, Value};

pub fn create_app() -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/webhook/telegram", post(telegram_webhook))
}

async fn health() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}

async fn telegram_webhook(Json(update): Json<Value>) -> Json<Value> {
    let update_id = update.get("update_id").and_then(Value::as_i64);
    tracing::info!(?update_id, "telegram update received");
    Json(json!({ "ok": true }))
}
