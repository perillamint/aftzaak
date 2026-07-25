mod role;
mod user;

use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::middleware::from_fn_with_state;
use axum::routing::{delete, get};
use axum::{Json, Router};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter,
    QuerySelect,
};
use uuid::Uuid;

use crate::AppState;
use crate::api::middleware::permission_filter::check_perm;
use crate::entity::{
    role::Entity as RoleEntity,
    role::ActiveModel as RoleActiveModel,
    user::Entity as UserEntity,
    user_role::Entity as UserRoleEntity,
    user_role::Column as UserRoleColumn,
    user_role::ActiveModel as UserRoleActiveModel,
};
use crate::error::{AppError, AppResult};
use crate::perms::{Permission, permissions_to_strings};
use crate::types::api::admin::{AssignRoleRequest, RoleInfo, RoleInfoPatch};
use crate::types::api::{ListQuery, ListResponse};

pub fn get_router() -> Router<Arc<AppState>> {
    Router::new()
        // Role CRUD
        .route("/roles", get(list_roles).post(create_role))
        .route(
            "/roles/{id}",
            get(get_role).patch(update_role).delete(delete_role),
        )
        // User-role assignment
        .route(
            "/users/{user_id}/roles",
            get(list_user_roles).post(assign_role),
        )
        .route("/users/{user_id}/roles/{role_id}", delete(revoke_role))
        .layer(from_fn_with_state(Permission::AdminManage, check_perm))
}

// --- Role CRUD ---

async fn list_roles(
    State(state): State<Arc<AppState>>,
    axum::extract::Query(q): axum::extract::Query<ListQuery>,
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

    if let Some(v) = req.name {
        active.name = ActiveValue::Set(v);
    }
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

// --- User-role assignment ---

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
