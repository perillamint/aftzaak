use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use struct_patch::Patch;
use uuid::Uuid;

use crate::entity::role::Model as RoleModel;
use crate::perms::{Permission, parse_permissions};

#[derive(Serialize, Deserialize, Patch)]
#[patch(attribute(derive(Deserialize)))]
pub struct RoleInfo {
    pub id: Uuid,
    pub name: String,
    pub permissions: Vec<Permission>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

impl TryFrom<RoleModel> for RoleInfo {
    type Error = strum::ParseError;

    fn try_from(m: RoleModel) -> Result<Self, Self::Error> {
        Ok(Self {
            id: m.id,
            name: m.name,
            permissions: parse_permissions(m.permissions)?,
            created_at: m.created_at,
            updated_at: m.updated_at,
        })
    }
}

#[derive(Deserialize)]
pub struct AssignRoleRequest {
    pub role_id: Uuid,
}
