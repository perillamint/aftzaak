use std::sync::Arc;

use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, PasswordHasher};
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::routing::{delete, get};
use axum::{Json, Router};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, QuerySelect,
};
use uuid::Uuid;

use crate::AppState;
use crate::entity::{
    role::Entity as RoleEntity, user::ActiveModel as UserActiveModel, user::Column as UserColumn,
    user::Entity as UserEntity, user_role::ActiveModel as UserRoleActiveModel,
    user_role::Column as UserRoleColumn, user_role::Entity as UserRoleEntity,
};
use crate::error::{AppError, AppResult};
use crate::types::api::admin::{AssignRoleRequest, RoleInfo};
use crate::types::api::auth::{UserInfo, UserInfoPatch};
use crate::types::api::{ListQuery, ListResponse};

pub fn get_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(list_users).post(create_user))
        .route("/{user_id}", get(get_user))
        .route("/{user_id}/roles", get(list_user_roles).post(assign_role))
        .route("/{user_id}/roles/{role_id}", delete(revoke_role))
}

async fn list_users(
    State(state): State<Arc<AppState>>,
    Query(q): Query<ListQuery>,
) -> AppResult<Json<ListResponse<UserInfo>>> {
    let limit = q.limit.unwrap_or(20).min(100);
    let offset = q.offset.unwrap_or(0);

    let total = UserEntity::find().count(&state.db).await?;

    let users = UserEntity::find()
        .order_by_desc(UserColumn::CreatedAt)
        .offset(offset)
        .limit(limit)
        .all(&state.db)
        .await?
        .into_iter()
        .map(UserInfo::from)
        .collect();

    Ok(Json(ListResponse { data: users, total }))
}

async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(req): Json<UserInfoPatch>,
) -> AppResult<(StatusCode, Json<UserInfo>)> {
    let email = req
        .email
        .ok_or_else(|| AppError::BadRequest("Empty email".to_string()))?;

    // Check for existing user
    let existing = UserEntity::find()
        .filter(UserColumn::Email.eq(&email))
        .one(&state.db)
        .await?;
    if existing.is_some() {
        return Err(AppError::UserExists);
    }

    let hashed = match req.password.flatten() {
        Some(password) => {
            let salt = SaltString::generate(&mut OsRng);
            let hash = Argon2::default()
                .hash_password(password.as_bytes(), &salt)
                .map_err(|e| AppError::PasswordHash(e.to_string()))?;
            Some(hash.to_string())
        }
        None => None,
    };

    let now = Utc::now().fixed_offset();
    let model = UserActiveModel {
        id: ActiveValue::Set(Uuid::now_v7()),
        email: ActiveValue::Set(email),
        password: ActiveValue::Set(hashed),
        display_name: ActiveValue::Set(req.display_name.flatten()),
        state: ActiveValue::Set(req.state.unwrap_or(true)),
        created_at: ActiveValue::Set(now),
        updated_at: ActiveValue::Set(now),
    };
    let model = model.insert(&state.db).await?;
    Ok((StatusCode::CREATED, Json(model.into())))
}

async fn get_user(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<Uuid>,
) -> AppResult<Json<UserInfo>> {
    let model = UserEntity::find_by_id(user_id)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound("user".to_string()))?;
    Ok(Json(model.into()))
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
