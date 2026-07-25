use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumString};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, AsRefStr, EnumString, Display,
)]
pub enum Permission {
    #[serde(rename = "item:read")]
    #[strum(serialize = "item:read")]
    ItemRead,

    #[serde(rename = "item:write")]
    #[strum(serialize = "item:write")]
    ItemWrite,

    #[serde(rename = "item:delete")]
    #[strum(serialize = "item:delete")]
    ItemDelete,

    #[serde(rename = "facet:read")]
    #[strum(serialize = "facet:read")]
    FacetRead,

    #[serde(rename = "facet:write")]
    #[strum(serialize = "facet:write")]
    FacetWrite,

    #[serde(rename = "facet:delete")]
    #[strum(serialize = "facet:delete")]
    FacetDelete,

    #[serde(rename = "admin:manage")]
    #[strum(serialize = "admin:manage")]
    AdminManage,
}

/// Convert entity's `Vec<String>` → `Vec<Permission>`.
///
/// Returns an error if any string doesn't match a known permission, so a
/// stale token could be rejected properly.
pub fn parse_permissions(raw: Vec<String>) -> Result<Vec<Permission>, strum::ParseError> {
    raw.iter().map(|s| s.parse()).collect()
}

/// Convert `Vec<Permission>` → `Vec<String>` for entity writes.
pub fn permissions_to_strings(perms: &[Permission]) -> Vec<String> {
    perms.iter().map(|p| p.to_string()).collect()
}
