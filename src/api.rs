use std::sync::Arc;
use axum::Router;
use crate::AppState;

mod v1;

pub fn get_router() -> axum::Router<Arc<AppState>> {
    Router::new()
        .nest("/v1", v1::get_router())
}
