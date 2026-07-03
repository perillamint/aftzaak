use std::sync::Arc;
use axum::{Json, Router};
use axum::routing::get;
use crate::AppState;
use crate::types::api::Heartbeat;

async fn heartbeat() -> Json<Heartbeat> {
    Json(Heartbeat::default())
}

pub fn get_router() -> axum::Router<Arc<AppState>> {
    Router::new()
        .route("/heartbeat", get(heartbeat))
}
