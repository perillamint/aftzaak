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
