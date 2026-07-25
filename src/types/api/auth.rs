use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use struct_patch::Patch;
use uuid::Uuid;

use crate::entity::user::Model as UserModel;
use crate::perms::Permission;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub display_name: String,
}

#[derive(Serialize, Deserialize, Patch)]
#[patch(attribute(derive(Deserialize)))]
pub struct UserInfo {
    pub id: Uuid,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: Option<String>,
    pub display_name: Option<String>,
    pub state: bool,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

impl From<UserModel> for UserInfo {
    fn from(m: UserModel) -> Self {
        Self {
            id: m.id,
            email: m.email,
            password: m.password,
            display_name: m.display_name,
            state: m.state,
            created_at: m.created_at,
            updated_at: m.updated_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenPayload {
    pub perm: Vec<Permission>,
}

#[derive(Serialize, Deserialize)]
pub struct TokenResponse {
    pub token: String,
    pub token_type: String,
    pub exp: i64,
}

#[derive(Serialize)]
pub struct TokenInfo {
    pub user_id: String,
    pub perm: Vec<Permission>,
}
