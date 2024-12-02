use serde::{Deserialize, Serialize};

use crate::{
    domain::chat::{Chat, ChatRepo, ChatType, Msg},
    error::AppError,
};

use super::auth::ClaimUser;

pub struct ChatService {
    repo: Box<dyn ChatRepo + Send + Sync>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateChatDto {
    pub name: Option<String>,
    pub chat_type: ChatType,
    pub members: Vec<u64>,
}
impl From<Chat> for CreateChatDto {
    fn from(chat: Chat) -> Self {
        Self {
            name: chat.name,
            chat_type: chat.chat_type,
            members: chat.members.into_iter().map(|uid| uid as u64).collect(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateChatDto {
    pub name: Option<String>,
    pub chat_type: ChatType,
    pub members: Vec<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendMsgDto {
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct ListOptionsDto {
    pub last_id: Option<u64>,
    pub limit: Option<u8>,
}

impl ChatService {
    pub fn new(repo: Box<dyn ChatRepo + Send + Sync>) -> Self {
        Self { repo }
    }

    pub async fn get_by_id(&self, chat_id: u64) -> Result<Option<Chat>, AppError> {
        self.repo.extract_by_id(chat_id as i64).await
    }

    pub async fn delete(&self, chat_id: u64) -> Result<Chat, AppError> {
        self.repo.delete_by_id(chat_id as i64).await
    }

    pub async fn update(&self, chat_id: u64, input: UpdateChatDto) -> Result<Chat, AppError> {
        let mut chat = self.repo.extract_by_id(chat_id as i64).await?.unwrap();
        chat.name = input.name;
        chat.chat_type = input.chat_type;
        chat.members = input.members.into_iter().map(|v| v as i64).collect();
        self.repo.save(&chat).await
    }
    pub async fn create(
        &self,
        user_id: u64,
        ws_id: u64,
        input: CreateChatDto,
    ) -> Result<Chat, AppError> {
        // validate
        if !input.members.contains(&user_id) {
            return Err(AppError::InvalidError(
                "current user should in members".to_owned(),
            ));
        }
        if !self
            .repo
            .is_members_exist(input.members.iter().map(|v| *v as i64).collect())
            .await?
        {
            return Err(AppError::InvalidError("members should exist".to_owned()));
        }
        let input = Chat::new(ws_id as i64, input.name, input.chat_type, input.members);
        self.repo.save(&input).await
    }

    pub async fn list_all(&self, user_id: u64, ws_id: u64) -> Result<Vec<Chat>, AppError> {
        self.repo
            .extract_all_chat(user_id as i64, ws_id as i64)
            .await
    }

    pub async fn send_msg(
        &self,
        user: &ClaimUser,
        chat_id: u64,
        input: SendMsgDto,
    ) -> Result<Msg, AppError> {
        let input = Msg::new(chat_id as _, user.id as _, input.content);
        self.repo.store_msg(&input).await
    }

    pub async fn list_messages(
        &self,
        chat_id: u64,
        last_id: u64,
        limit: u64,
    ) -> Result<Vec<Msg>, AppError> {
        self.repo
            .extract_messages(chat_id as i64, last_id as i64, limit as i64)
            .await
    }
}
