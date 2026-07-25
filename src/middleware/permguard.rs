use axum::extract::{Request, State};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};

use crate::error::AppError;
use crate::perms::Permission;
use crate::util::tokensigner::Claims;

/// Permission check middleware.
///
/// Used with `from_fn_with_state(perm, check_perm)` on individual routes
/// or router groups. The required `Permission` is passed as the layer's state.
pub async fn check_perm(State(perm): State<Permission>, req: Request, next: Next) -> Response {
    let result = req
        .extensions()
        .get::<Claims>()
        .ok_or(AppError::Unauthorized)
        .and_then(|c| {
            if c.custom_claim.perm.iter().any(|p| *p == perm) {
                Ok(())
            } else {
                Err(AppError::Forbidden)
            }
        });

    match result {
        Ok(()) => next.run(req).await,
        Err(e) => e.into_response(),
    }
}
