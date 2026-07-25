use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{delete, get};
use axum::{Json, Router};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, QueryFilter};
use uuid::Uuid;

use crate::AppState;
use crate::entity::{
    role::Entity as RoleEntity, user::Entity as UserEntity,
    user_role::ActiveModel as UserRoleActiveModel, user_role::Column as UserRoleColumn,
    user_role::Entity as UserRoleEntity,
};
use crate::error::{AppError, AppResult};
use crate::types::api::admin::{AssignRoleRequest, RoleInfo};

pub fn get_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/{user_id}/roles", get(list_user_roles).post(assign_role))
        .route("/{user_id}/roles/{role_id}", delete(revoke_role))
}

async fn list_user_roles(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<Uuid>,
) -> AppResult<Json<Vec<RoleInfo>>> {
    // Verify user exists
    let _user = UserEntity::find_by_id(user_id)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound("user".to_string()))?;

    let user_roles = UserRoleEntity::find()
        .filter(UserRoleColumn::UserId.eq(user_id))
        .find_also_related(RoleEntity)
        .all(&state.db)
        .await?;

    let roles = user_roles
        .into_iter()
        .filter_map(|(_, r)| r)
        .map(RoleInfo::try_from)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(Json(roles))
}

async fn assign_role(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<Uuid>,
    Json(req): Json<AssignRoleRequest>,
) -> AppResult<StatusCode> {
    // Verify user exists
    let _user = UserEntity::find_by_id(user_id)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound("user".to_string()))?;

    // Verify role exists
    let _role = RoleEntity::find_by_id(req.role_id)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound("role".to_string()))?;

    // Check if already assigned
    let existing = UserRoleEntity::find()
        .filter(UserRoleColumn::UserId.eq(user_id))
        .filter(UserRoleColumn::RoleId.eq(req.role_id))
        .one(&state.db)
        .await?;

    if existing.is_some() {
        return Ok(StatusCode::CONFLICT);
    }

    let now = Utc::now().fixed_offset();
    let model = UserRoleActiveModel {
        id: ActiveValue::Set(Uuid::now_v7()),
        user_id: ActiveValue::Set(user_id),
        role_id: ActiveValue::Set(req.role_id),
        created_at: ActiveValue::Set(now),
    };
    model.insert(&state.db).await?;

    Ok(StatusCode::CREATED)
}

async fn revoke_role(
    State(state): State<Arc<AppState>>,
    Path((user_id, role_id)): Path<(Uuid, Uuid)>,
) -> AppResult<StatusCode> {
    let model = UserRoleEntity::find()
        .filter(UserRoleColumn::UserId.eq(user_id))
        .filter(UserRoleColumn::RoleId.eq(role_id))
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound("user_role".to_string()))?;

    UserRoleEntity::delete_by_id(model.id)
        .exec(&state.db)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}
