use std::sync::Arc;

use axum::Router;

use crate::AppState;

mod v1;

pub fn get_router(state: Arc<AppState>) -> axum::Router<Arc<AppState>> {
    Router::new().nest("/v1", v1::get_router(state))
}
