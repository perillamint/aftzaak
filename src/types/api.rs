pub mod auth;
pub mod facet;
pub mod item;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Heartbeat {
    pub msg: String,
}

impl Default for Heartbeat {
    fn default() -> Self {
        Self {
            msg: "Up and running.".to_string(),
        }
    }
}

/// Common pagination query for list endpoints.
#[derive(Deserialize)]
pub struct ListQuery {
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

/// Common paginated list response.
#[derive(Serialize)]
pub struct ListResponse<T: Serialize> {
    pub data: Vec<T>,
    pub total: u64,
}
