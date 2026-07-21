use crate::AppState;
use axum::Router;
use std::sync::Arc;

pub fn get_router() -> Router<Arc<AppState>> {
    Router::new()
}
