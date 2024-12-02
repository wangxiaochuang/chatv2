use axum::async_trait;
use sqlx::PgPool;

use crate::{
    domain::chat::{Chat, ChatRepo, Msg},
    error::AppError,
};

#[derive(Clone)]
pub struct ChatRepoImpl {
    pool: PgPool,
}

impl ChatRepoImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ChatRepo for ChatRepoImpl {
    async fn delete_by_id(&self, chat_id: i64) -> Result<Chat, AppError> {
        let chat = sqlx::query_as(
            r#"
            UPDATE chats set status = 0
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(chat_id)
        .fetch_one(&self.pool)
        .await?;
        Ok(chat)
    }
    async fn extract_by_id(&self, chat_id: i64) -> Result<Option<Chat>, AppError> {
        let chats = sqlx::query_as(
            r#"
            SELECT id, ws_id, name, chat_type, members, status, created_at
            FROM chats
            WHERE id = $1
            "#,
        )
        .bind(chat_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(chats)
    }
    async fn store_msg(&self, input: &Msg) -> Result<Msg, AppError> {
        let message: Msg = sqlx::query_as(
            r#"
          INSERT INTO messages (chat_id, sender_id, content)
          VALUES ($1, $2, $3)
          RETURNING *
          "#,
        )
        .bind(input.chat_id as i64)
        .bind(input.sender_id as i64)
        .bind(&input.content)
        .fetch_one(&self.pool)
        .await?;

        Ok(message)
    }
    async fn save(&self, input: &Chat) -> Result<Chat, AppError> {
        let chat = if input.id == -1 {
            sqlx::query_as(
                r#"
        INSERT INTO chats (ws_id, name, chat_type, members)
            VALUES ($1, $2, $3, $4)
            RETURNING *
        "#,
            )
            .bind(&input.ws_id)
            .bind(&input.name)
            .bind(&input.chat_type)
            .bind(&input.members)
            .fetch_one(&self.pool)
            .await?
        } else {
            sqlx::query_as(
                r#"
                UPDATE chats SET name=$1,chat_type=$2
                WHERE id=$3
                RETURNING *
                "#,
            )
            .bind(&input.name)
            .bind(&input.chat_type)
            .bind(&input.id)
            .fetch_one(&self.pool)
            .await?
        };
        Ok(chat)
    }
    async fn is_members_exist(&self, members: Vec<i64>) -> Result<bool, AppError> {
        let count: i64 = sqlx::query_scalar(
            r#"
        SELECT count(*) as count
        FROM users
        WHERE id = ANY($1)
        "#,
        )
        .bind(&members)
        .fetch_one(&self.pool)
        .await?;
        Ok(count as usize == members.len())
    }

    async fn extract_all_chat(&self, user_id: i64, ws_id: i64) -> Result<Vec<Chat>, AppError> {
        let chats = sqlx::query_as(
            r#"
            SELECT *
            FROM chats
            WHERE ws_id = $1 AND $2 = ANY(members) AND status = 1
            "#,
        )
        .bind(ws_id as i64)
        .bind(user_id as i64)
        .fetch_all(&self.pool)
        .await?;

        Ok(chats)
    }

    async fn extract_messages(
        &self,
        chat_id: i64,
        last_id: i64,
        limit: i64,
    ) -> Result<Vec<Msg>, AppError> {
        let msgs = sqlx::query_as(
            r#"
            SELECT *
            FROM messages
            WHERE chat_id = $1
            AND id < $2
            ORDER BY id DESC
            limit $3
            "#,
        )
        .bind(chat_id)
        .bind(last_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(msgs)
    }
}
