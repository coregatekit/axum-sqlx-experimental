use axum::{response::IntoResponse, Json};
use serde_json::json;

pub async fn health_check_handler() -> impl IntoResponse {
    const HEALTH_CHECK_RESPONSE: &str = "Server is healthy! 😎";

    Json(json!({
        "status": "ok",
        "message": HEALTH_CHECK_RESPONSE,
    }))
}
