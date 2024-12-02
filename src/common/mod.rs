use serde::{Deserialize, Serialize};

pub mod utils;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthInfo {
    pub token: String,
}
