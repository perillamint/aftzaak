use crate::AppState;
use crate::types::api::Heartbeat;
use axum::routing::get;
use axum::{Json, Router};
use std::sync::Arc;

mod auth;

async fn heartbeat() -> Json<Heartbeat> {
    Json(Heartbeat::default())
}

pub fn get_router() -> axum::Router<Arc<AppState>> {
    Router::new()
        .route("/heartbeat", get(heartbeat))
        .nest("/auth", auth::get_router())
}
