use serde::Serialize;

use crate::{
    domain::user::{User, UserRepo},
    error::AppError,
};

pub struct UserService {
    repo: Box<dyn UserRepo + Send + Sync>,
}

#[derive(Debug, Serialize)]
pub struct ChatUserDto {
    pub id: u64,
    pub display: String,
}
impl From<User> for ChatUserDto {
    fn from(u: User) -> Self {
        Self {
            id: u.id as _,
            display: u.fullname,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct UserInfoDto {
    pub id: u64,
    pub display: String,
    pub workspace: String,
}

impl UserService {
    pub fn new(repo: Box<dyn UserRepo + Send + Sync>) -> Self {
        Self { repo }
    }

    pub async fn fetch_all_in_ws(&self, ws_id: u64) -> Result<Vec<ChatUserDto>, AppError> {
        Ok(self
            .repo
            .extract_all_users(ws_id)
            .await?
            .into_iter()
            .map(ChatUserDto::from)
            .collect::<Vec<_>>())
    }

    pub async fn get_user_info(&self, user_id: u64) -> Result<Option<UserInfoDto>, AppError> {
        let user = match self.repo.find_by_id(user_id as _).await? {
            Some(user) => user,
            None => return Ok(None),
        };
        let ws = match self.repo.find_ws_by_id(user.ws_id).await? {
            Some(ws) => ws,
            None => return Ok(None),
        };
        Ok(Some(UserInfoDto {
            id: user.id as _,
            display: user.fullname,
            workspace: ws.name,
        }))
    }
}
