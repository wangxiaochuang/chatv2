use axum::async_trait;
use sqlx::PgPool;

use crate::{
    domain::user::{User, UserRepo, Workspace},
    error::AppError,
};

#[derive(Clone)]
pub struct UserRepoImpl {
    pool: PgPool,
}

impl UserRepoImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepo for UserRepoImpl {
    async fn extract_all_users(&self, ws_id: u64) -> Result<Vec<User>, AppError> {
        let users = sqlx::query_as(
            r#"
        SELECT *
        FROM users
        WHERE ws_id = $1
        "#,
        )
        .bind(ws_id as i64)
        .fetch_all(&self.pool)
        .await?;
        Ok(users)
    }
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as(
            "select id, ws_id, fullname, email, password_hash, created_at from users where email = $1",
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;
        Ok(user)
    }

    async fn find_by_id(&self, user_id: i64) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as("select * from users where id = $1")
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(user)
    }

    async fn save(&self, user: &User) -> Result<User, AppError> {
        let exist_user = self.find_by_email(&user.email).await?;
        if exist_user.is_some() {
            return Err(AppError::ConflictError(user.email.clone()));
        }
        let user: User = sqlx::query_as(
            r#"
        insert into users (ws_id, email, fullname, password_hash)
        values ($1, $2, $3, $4)
        returning id, ws_id, email, fullname, password_hash, created_at
        "#,
        )
        .bind(user.ws_id)
        .bind(&user.email)
        .bind(&user.fullname)
        .bind(&user.password_hash)
        .fetch_one(&self.pool)
        .await?;
        Ok(user)
    }

    async fn find_ws_by_name(&self, name: &str) -> Result<Option<Workspace>, AppError> {
        let ws = sqlx::query_as(
            r#"
        SELECT *
        FROM workspaces
        WHERE name = $1
        "#,
        )
        .bind(name)
        .fetch_optional(&self.pool)
        .await?;
        Ok(ws)
    }

    async fn find_ws_by_id(&self, ws_id: i64) -> Result<Option<Workspace>, AppError> {
        let ws = sqlx::query_as(
            r#"
        SELECT *
        FROM workspaces
        WHERE id = $1
        "#,
        )
        .bind(ws_id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(ws)
    }

    async fn save_ws(&self, ws: &Workspace) -> Result<Workspace, AppError> {
        let ws = sqlx::query_as(
            r#"
        INSERT INTO workspaces (name, owner_id)
        VALUES ($1, $2)
        RETURNING id, name, owner_id, created_at
        "#,
        )
        .bind(&ws.name)
        .bind(ws.owner_id)
        .fetch_one(&self.pool)
        .await?;
        Ok(ws)
    }
}
