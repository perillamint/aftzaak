use serde::{Deserialize, Serialize};
use struct_patch::Patch;
use uuid::Uuid;

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
pub struct UserInfo {
    pub id: Uuid,
    pub email: String,
    pub password: Option<String>,
    pub display_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenPayload {
    pub perm: String,
}

#[derive(Serialize, Deserialize)]
pub struct TokenResponse {
    pub token: String,
    pub token_type: String,
    pub exp: i64,
}
