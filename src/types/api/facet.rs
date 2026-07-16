use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use struct_patch::Patch;
use uuid::Uuid;

use crate::entity::facet;

#[derive(Serialize, Deserialize, Patch)]
#[patch(attribute(derive(Deserialize)))]
pub struct Facet {
    pub id: Uuid,
    pub key: String,
    pub display_name: String,
    pub value_type: String,
    pub is_multi_value: bool,
    pub sort_order: i32,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

impl From<facet::Model> for Facet {
    fn from(m: facet::Model) -> Self {
        Self {
            id: m.id,
            key: m.key,
            display_name: m.display_name,
            value_type: m.value_type,
            is_multi_value: m.is_multi_value,
            sort_order: m.sort_order,
            created_at: m.created_at,
            updated_at: m.updated_at,
        }
    }
}

#[derive(Deserialize)]
pub struct FacetListQuery {
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

#[derive(Serialize)]
pub struct FacetListResponse {
    pub facets: Vec<Facet>,
    pub total: u64,
}
