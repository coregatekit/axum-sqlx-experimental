use axum::{routing::get, Router};
use sqlx::{MySql, Pool};
use std::sync::Arc;

use crate::{handlers::health_checks::health_check_handler, models::app_state::AppState};

pub fn create_router(pool: Pool<MySql>) -> Router {
    Router::new()
        .with_state(Arc::new(AppState { db: pool.clone() }))
        .route("/health", get(health_check_handler))
}
