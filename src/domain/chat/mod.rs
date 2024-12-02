use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

mod repo;

pub use repo::ChatRepo;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "chat_type", rename_all = "snake_case")]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub enum ChatType {
    Single,
    Group,
    PrivateChannel,
    PublicChannel,
}

impl Default for ChatType {
    fn default() -> Self {
        ChatType::Single
    }
}

#[derive(Debug, Default, FromRow, PartialEq, Serialize, Deserialize)]
pub struct Chat {
    pub id: i64,
    pub ws_id: i64,
    pub name: Option<String>,
    pub chat_type: ChatType,
    pub members: Vec<i64>,
    pub status: i16,
    pub created_at: DateTime<Utc>,
}

impl Chat {
    pub fn new(ws_id: i64, name: Option<String>, chat_type: ChatType, members: Vec<u64>) -> Self {
        Self {
            id: -1,
            ws_id,
            name,
            chat_type,
            members: members.into_iter().map(|x| x as i64).collect(),
            status: 1,
            created_at: Utc::now(),
        }
    }
}

#[derive(Debug, Default, FromRow, PartialEq, Serialize, Deserialize)]
pub struct Msg {
    pub id: i64,
    pub chat_id: i64,
    pub sender_id: i64,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

impl Msg {
    pub fn new(chat_id: i64, sender_id: i64, content: String) -> Self {
        Self {
            id: -1,
            chat_id,
            sender_id,
            content,
            created_at: Utc::now(),
        }
    }
}
