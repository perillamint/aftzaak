use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait, PaginatorTrait, QuerySelect};
use uuid::Uuid;

use crate::entity::{role::ActiveModel as RoleActiveModel, role::Entity as RoleEntity};
use crate::error::{AppError, AppResult};
use crate::perms::permissions_to_strings;
use crate::types::api::admin::{RoleInfo, RoleInfoPatch};
use crate::types::api::{ListQuery, ListResponse};
use crate::{AppState, update_am};

pub fn get_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(list_roles).post(create_role))
        .route(
            "/{id}",
            get(get_role).patch(update_role).delete(delete_role),
        )
}

async fn list_roles(
    State(state): State<Arc<AppState>>,
    Query(q): Query<ListQuery>,
) -> AppResult<Json<ListResponse<RoleInfo>>> {
    let limit = q.limit.unwrap_or(20).min(100);
    let offset = q.offset.unwrap_or(0);

    let total = RoleEntity::find().count(&state.db).await?;

    let roles = RoleEntity::find()
        .offset(offset)
        .limit(limit)
        .all(&state.db)
        .await?
        .into_iter()
        .map(RoleInfo::try_from)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(Json(ListResponse { data: roles, total }))
}

async fn create_role(
    State(state): State<Arc<AppState>>,
    Json(req): Json<RoleInfoPatch>,
) -> AppResult<(StatusCode, Json<RoleInfo>)> {
    let now = Utc::now().fixed_offset();
    let model = RoleActiveModel {
        id: ActiveValue::Set(Uuid::now_v7()),
        name: ActiveValue::Set(
            req.name
                .ok_or_else(|| AppError::BadRequest("Empty name".to_string()))?,
        ),
        permissions: ActiveValue::Set(permissions_to_strings(&req.permissions.unwrap_or_default())),
        created_at: ActiveValue::Set(now),
        updated_at: ActiveValue::Set(now),
    };
    let model = model.insert(&state.db).await?;
    Ok((StatusCode::CREATED, Json(model.try_into()?)))
}

async fn get_role(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<RoleInfo>> {
    let model = RoleEntity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound("role".to_string()))?;
    Ok(Json(model.try_into()?))
}

async fn update_role(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(req): Json<RoleInfoPatch>,
) -> AppResult<Json<RoleInfo>> {
    let model = RoleEntity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound("role".to_string()))?;

    let mut active: RoleActiveModel = model.into();
    let now = Utc::now().fixed_offset();

    update_am!(active, req, name);
    if let Some(v) = req.permissions {
        active.permissions = ActiveValue::Set(permissions_to_strings(&v));
    }
    active.updated_at = ActiveValue::Set(now);

    let model = active.update(&state.db).await?;
    Ok(Json(model.try_into()?))
}

async fn delete_role(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> AppResult<StatusCode> {
    RoleEntity::delete_by_id(id).exec(&state.db).await?;
    Ok(StatusCode::NO_CONTENT)
}
