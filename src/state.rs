use anyhow::{bail, Result};
use sqlx::postgres::PgPoolOptions;
use std::{env, fmt, fs::File, io::Read, ops::Deref, sync::Arc, time::Duration};

use serde::{Deserialize, Serialize};

use crate::{
    adapter::driving::db::{chat::ChatRepoImpl, user::UserRepoImpl},
    common::utils::token::TokenSignVerify,
    service::{
        chat::ChatService, file::FileService, notif::NotifService, user::UserService, AuthService,
    },
};

#[derive(Deserialize, Serialize, Debug)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub auth: AuthConfig,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AuthConfig {
    pub sk: String,
    pub pk: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub db_url: String,
    pub upload_base_dir: String,
}

impl ServerConfig {
    pub fn local_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

const LOCAL_CONFIG: &str = "app.yml";
const ENV_CONFIG_VAR: &str = "CHAT_CONFIG";
const SYSTEM_CONFIG: &str = "/etc/config/app.yml";

fn get_config_reader() -> Result<File, anyhow::Error> {
    let reader = match (
        File::open(LOCAL_CONFIG),
        env::var(ENV_CONFIG_VAR),
        File::open(SYSTEM_CONFIG),
    ) {
        (Ok(reader), _, _) => reader,
        (_, Ok(path), _) => File::open(path)?,
        (_, _, Ok(reader)) => reader,
        _ => bail!("No config file found"),
    };
    Ok(reader)
}

impl AppConfig {
    pub fn try_load() -> Result<Self> {
        Self::try_load_from(get_config_reader()?)
    }
    pub fn try_load_from<R: Read>(reader: R) -> Result<Self> {
        Ok(serde_yaml::from_reader(reader)?)
    }
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub inner: Arc<AppStateInner>,
}

impl AppState {
    pub async fn try_new_from_conf() -> Result<Self> {
        Self::try_new(AppConfig::try_load()?).await
    }
    pub async fn try_new(config: AppConfig) -> Result<Self> {
        let pool = PgPoolOptions::new()
            .acquire_timeout(Duration::from_millis(1000))
            .connect(&config.server.db_url)
            .await?;
        let tokensv = TokenSignVerify::try_new(&config.auth.pk, &config.auth.sk)?;
        let user_repo = Box::new(UserRepoImpl::new(pool.clone()));
        let chat_repo = Box::new(ChatRepoImpl::new(pool.clone()));
        let file_svc = FileService::new(&config.server.upload_base_dir);
        let notif_svc = NotifService::try_new(&config.server.db_url).await?;
        Ok(Self {
            inner: Arc::new(AppStateInner {
                config,
                auth: AuthService::new(user_repo.clone(), tokensv),
                user: UserService::new(user_repo),
                chat: ChatService::new(chat_repo),
                file: file_svc,
                notif: notif_svc,
            }),
        })
    }
}

pub struct AppStateInner {
    pub config: AppConfig,
    pub auth: AuthService,
    pub user: UserService,
    pub chat: ChatService,
    pub file: FileService,
    pub notif: NotifService,
}

impl fmt::Debug for AppStateInner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AppStateInner")
            .field("config", &self.config)
            .finish()
    }
}

impl Deref for AppState {
    type Target = AppStateInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
