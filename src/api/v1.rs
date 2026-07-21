use std::sync::Arc;

use axum::Extension;
use axum::Json;
use axum::Router;
use axum::middleware::from_fn_with_state;
use axum::routing::get;

use crate::AppState;
use crate::middleware::tokenauth;
use crate::types::api::Heartbeat;
use crate::types::api::auth::TokenInfo;
use crate::util::tokensigner::Claims;

mod admin;
mod auth;
mod facet;
mod item;

async fn heartbeat() -> Json<Heartbeat> {
    Json(Heartbeat::default())
}

async fn me(Extension(claims): Extension<Claims>) -> Json<TokenInfo> {
    Json(TokenInfo {
        user_id: claims.sub,
        perm: claims.custom_claim.perm,
    })
}

pub fn get_router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    let protected = Router::new()
        .route("/me", get(me))
        .nest("/item", item::get_router())
        .nest("/facet", facet::get_router())
        .nest("/admin", admin::get_router())
        .layer(from_fn_with_state(state, tokenauth::require_auth));

    Router::new()
        .route("/heartbeat", get(heartbeat))
        .nest("/auth", auth::get_router())
        .merge(protected)
}
