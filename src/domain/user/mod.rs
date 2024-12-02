use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

mod repo;

pub use repo::UserRepo;

#[derive(Debug, Clone, Default, FromRow, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub ws_id: i64,
    pub fullname: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Default, FromRow, PartialEq, Serialize, Deserialize)]
pub struct Workspace {
    pub id: i64,
    pub name: String,
    pub owner_id: i64,
    pub created_at: DateTime<Utc>,
}
