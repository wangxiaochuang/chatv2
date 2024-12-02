use serde::{Deserialize, Serialize};

use crate::{
    common::utils::{hash_passwd, token::TokenSignVerify, verify_passwd},
    domain::user::{User, UserRepo, Workspace},
    error::AppError,
};

pub struct AuthService {
    repo: Box<dyn UserRepo + Send + Sync>,
    tokensv: TokenSignVerify,
}

impl AuthService {
    pub fn new(repo: Box<dyn UserRepo + Send + Sync>, tokensv: TokenSignVerify) -> Self {
        Self { repo, tokensv }
    }

    pub fn verify_token(&self, token: impl AsRef<str>) -> Result<ClaimUser, AppError> {
        self.tokensv.verify(token.as_ref())
    }

    pub async fn signin(&self, user: SigninUserDto) -> Result<String, AppError> {
        let user_in_db = match self.repo.find_by_email(&user.email).await? {
            Some(user) => user,
            None => return Err(AppError::NotFound(user.email)),
        };
        verify_passwd(&user.password, &user_in_db.password_hash)?;
        self.tokensv.sign(ClaimUser::from(user_in_db.clone()))
    }

    pub async fn signup(&self, input: SignupUserDto) -> Result<String, AppError> {
        match self.repo.find_by_email(&input.email).await {
            Ok(Some(_)) => return Err(AppError::ConflictError("email already exist".to_owned())),
            Ok(None) => {}
            Err(e) => return Err(e),
        };

        let ws = match self.repo.find_ws_by_name(&input.workspace).await? {
            Some(ws) => ws,
            None => {
                let ws = Workspace {
                    name: input.workspace,
                    owner_id: 0,
                    ..Default::default()
                };
                self.repo.save_ws(&ws).await?
            }
        };

        let password_hash = hash_passwd(&input.password)?;
        let user = User {
            ws_id: ws.id,
            fullname: input.fullname,
            email: input.email,
            password_hash: password_hash,
            ..Default::default()
        };
        let user = self.repo.save(&user).await?;
        self.tokensv.sign(ClaimUser::from(user))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SigninUserDto {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimUser {
    pub id: u64,
    pub ws_id: u64,
}
impl From<User> for ClaimUser {
    fn from(user: User) -> Self {
        Self {
            id: user.id as _,
            ws_id: user.ws_id as _,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignupUserDto {
    pub fullname: String,
    pub email: String,
    pub workspace: String,
    pub password: String,
}
