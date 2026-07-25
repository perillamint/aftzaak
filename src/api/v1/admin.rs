mod role;
mod user;

use std::sync::Arc;

use axum::Router;
use axum::middleware::from_fn_with_state;

use crate::AppState;
use crate::api::middleware::permission_filter::check_perm;
use crate::perms::Permission;

pub fn get_router() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/user", user::get_router())
        .nest("/role", role::get_router())
        .layer(from_fn_with_state(Permission::AdminManage, check_perm))
}
