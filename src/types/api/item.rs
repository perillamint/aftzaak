use sea_orm::prelude::{DateTimeWithTimeZone, Json};
use serde::{Deserialize, Serialize};
use struct_patch::Patch;
use uuid::Uuid;

use crate::entity::item;

#[derive(Serialize, Deserialize, Patch)]
#[patch(attribute(derive(Deserialize)))]
pub struct Item {
    pub id: Uuid,
    pub title: String,
    pub mime_type: String,
    pub size_bytes: i64,
    pub checksum: Option<String>,
    pub storage_uri: String,
    pub metadata: Option<Json>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

impl From<item::Model> for Item {
    fn from(m: item::Model) -> Self {
        Self {
            id: m.id,
            title: m.title,
            mime_type: m.mime_type,
            size_bytes: m.size_bytes,
            checksum: m.checksum,
            storage_uri: m.storage_uri,
            metadata: m.metadata,
            created_at: m.created_at,
            updated_at: m.updated_at,
        }
    }
}
