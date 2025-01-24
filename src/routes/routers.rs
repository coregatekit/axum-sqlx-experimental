use axum::{routing::get, Router};

use crate::handlers::health_checks::health_check_handler;

pub fn create_router() -> Router {
    Router::new().route("/health", get(health_check_handler))
}
