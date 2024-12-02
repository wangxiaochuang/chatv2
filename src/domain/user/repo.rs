use axum::async_trait;

use crate::error::AppError;

use super::{User, Workspace};

#[async_trait]
pub trait UserRepo {
    async fn extract_all_users(&self, ws_id: u64) -> Result<Vec<User>, AppError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError>;
    async fn find_by_id(&self, user_id: i64) -> Result<Option<User>, AppError>;
    async fn save(&self, input: &User) -> Result<User, AppError>;
    async fn find_ws_by_name(&self, name: &str) -> Result<Option<Workspace>, AppError>;
    async fn find_ws_by_id(&self, ws_id: i64) -> Result<Option<Workspace>, AppError>;
    async fn save_ws(&self, ws: &Workspace) -> Result<Workspace, AppError>;
}
