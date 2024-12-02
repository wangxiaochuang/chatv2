use axum::async_trait;

use crate::error::AppError;

use super::{Chat, Msg};

#[async_trait]
pub trait ChatRepo {
    async fn delete_by_id(&self, chat_id: i64) -> Result<Chat, AppError>;
    async fn extract_by_id(&self, chat_id: i64) -> Result<Option<Chat>, AppError>;
    async fn store_msg(&self, msg: &Msg) -> Result<Msg, AppError>;
    async fn save(&self, input: &Chat) -> Result<Chat, AppError>;
    async fn is_members_exist(&self, members: Vec<i64>) -> Result<bool, AppError>;
    async fn extract_all_chat(&self, user_id: i64, ws_id: i64) -> Result<Vec<Chat>, AppError>;
    async fn extract_messages(
        &self,
        chat_id: i64,
        last_id: i64,
        limit: i64,
    ) -> Result<Vec<Msg>, AppError>;
}
