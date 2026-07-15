use crate::AppState;
use axum::Router;
use std::sync::Arc;

mod v1;

pub fn get_router() -> axum::Router<Arc<AppState>> {
    Router::new().nest("/v1", v1::get_router())
}
