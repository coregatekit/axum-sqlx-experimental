use axum::{response::IntoResponse, routing::get, Json, Router};
use serde_json::json;

pub fn create_router() -> Router {
    Router::new().route("/health", get(health_check_handler))
}

pub async fn health_check_handler() -> impl IntoResponse {
    const HEALTH_CHECK_RESPONSE: &str = "Server is healthy! 😎";

    Json(json!({
        "status": "ok",
        "message": HEALTH_CHECK_RESPONSE,
    }))
}
