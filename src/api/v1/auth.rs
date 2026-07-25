use std::collections::HashSet;
use std::sync::Arc;

use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter};
use uuid::Uuid;

use crate::AppState;
use crate::entity::{
    role::Entity as RoleEntity,
    user::ActiveModel as UserActiveModel,
    user::Column as UserColumn,
    user::Entity as UserEntity,
    user_role::Column as UserRoleColumn,
    user_role::Entity as UserRoleEntity,
};
use crate::error::{AppError, AppResult};
use crate::perms::{Permission, parse_permissions};
use crate::types::api::auth::{LoginRequest, RegisterRequest, TokenPayload, TokenResponse};

pub fn get_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
}

async fn register(
    State(state): State<Arc<AppState>>,
    Json(req): Json<RegisterRequest>,
) -> AppResult<Json<()>> {
    let existing = UserEntity::find()
        .filter(UserColumn::Email.eq(&req.email))
        .one(&state.db)
        .await?;
    if existing.is_some() {
        return Err(AppError::UserExists);
    }

    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(req.password.as_bytes(), &salt)
        .map_err(|e| AppError::PasswordHash(e.to_string()))?;
    let hashed = hash.to_string();
    let now = Utc::now().fixed_offset();
    let model = UserActiveModel {
        id: sea_orm::ActiveValue::Set(Uuid::now_v7()),
        email: sea_orm::ActiveValue::Set(req.email.clone()),
        password: sea_orm::ActiveValue::Set(Some(hashed)),
        display_name: sea_orm::ActiveValue::Set(Some(req.display_name.clone())),
        state: sea_orm::ActiveValue::Set(true),
        created_at: sea_orm::ActiveValue::Set(now),
        updated_at: sea_orm::ActiveValue::Set(now),
    };
    let _user = model.insert(&state.db).await?;

    Ok(Json(()))
}

async fn login(
    State(state): State<Arc<AppState>>,
    Json(req): Json<LoginRequest>,
) -> AppResult<Json<TokenResponse>> {
    let user = UserEntity::find()
        .filter(UserColumn::Email.eq(&req.email))
        .filter(UserColumn::State.eq(true))
        .one(&state.db)
        .await?
        .ok_or(AppError::InvalidCredentials)?;

    let encoded = user
        .password
        .as_deref()
        .ok_or(AppError::InvalidCredentials)?;
    let parsed = PasswordHash::new(encoded).map_err(|e| AppError::PasswordHash(e.to_string()))?;
    if !Argon2::default()
        .verify_password(req.password.as_bytes(), &parsed)
        .is_ok()
    {
        return Err(AppError::InvalidCredentials);
    }

    let (token, exp) = {
        // Gather permissions from all roles assigned to this user
        let user_roles = UserRoleEntity::find()
            .filter(UserRoleColumn::UserId.eq(user.id))
            .find_also_related(RoleEntity)
            .all(&state.db)
            .await?;

        let perm: Vec<Permission> = {
            let raw: Vec<String> = user_roles
                .into_iter()
                .filter_map(|(_, r)| r)
                .flat_map(|r| r.permissions)
                .collect::<HashSet<_>>()
                .into_iter()
                .collect();
            parse_permissions(raw)?
        };

        state
            .tokensigner
            .sign(user.id.to_string(), TokenPayload { perm })?
    };

    Ok(Json(TokenResponse {
        token,
        token_type: "Bearer".to_string(),
        exp,
    }))
}
