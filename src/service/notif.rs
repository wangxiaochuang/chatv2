use std::{collections::HashSet, sync::Arc};

use dashmap::DashMap;
use futures::StreamExt as _;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgListener;
use tokio::sync::broadcast::{self, Receiver};
use tracing::{info, warn};

use crate::{
    domain::chat::{Chat, Msg},
    error::AppError,
};

type OnlineUserMap = DashMap<u64, broadcast::Sender<Arc<AppEvent>>>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "event")]
pub enum AppEvent {
    NewChat(Chat),
    AddToChat(Chat),
    RemoveFromChat(Chat),
    NewMessage(Msg),
}

impl AppEvent {
    pub fn get_name(&self) -> &'static str {
        match self {
            AppEvent::NewChat(_) => "new_chat",
            AppEvent::AddToChat(_) => "add_to_chat",
            AppEvent::RemoveFromChat(_) => "remove_from_chat",
            AppEvent::NewMessage(_) => "new_message",
        }
    }
}

pub struct NotifService {
    online_users: Arc<OnlineUserMap>,
}

// pg_notify('chat_updated', json_build_object('op', TG_OP, 'old', OLD, 'new', NEW)::text);
#[derive(Debug, Serialize, Deserialize)]
struct ChatUpdated {
    op: String,
    old: Option<Chat>,
    new: Option<Chat>,
}

impl ChatUpdated {
    fn get_affected_chat_user_ids(&self) -> HashSet<u64> {
        let old_users: HashSet<u64> = self
            .old
            .as_ref()
            .map(|chat| chat.members.iter().map(|v| *v as _).collect())
            .unwrap_or_default();
        let new_users: HashSet<u64> = self
            .new
            .as_ref()
            .map(|chat| chat.members.iter().map(|v| *v as _).collect())
            .unwrap_or_default();
        old_users.union(&new_users).cloned().collect()
    }
}

// pg_notify('chat_message_created', row_to_json(NEW)::text);
#[derive(Debug, Serialize, Deserialize)]
struct MessageCreated {
    message: Msg,
    members: Vec<i64>,
}

impl NotifService {
    pub async fn try_new(db_url: &str) -> Result<Self, AppError> {
        let online_users: Arc<OnlineUserMap> = Arc::new(Default::default());
        Self::listen(db_url, online_users.clone()).await?;
        Ok(Self { online_users })
    }

    pub fn register(&self, user_id: u64) -> Receiver<Arc<AppEvent>> {
        match self.online_users.get(&user_id) {
            Some(tx) => tx.subscribe(),
            None => {
                let (tx, rx) = broadcast::channel(1024);
                self.online_users.insert(user_id, tx);
                rx
            }
        }
    }

    async fn listen(db_url: &str, online_users: Arc<OnlineUserMap>) -> Result<(), AppError> {
        let mut listener = PgListener::connect(db_url).await?;
        listener.listen("chat_updated").await?;
        listener.listen("chat_message_created").await?;

        // { process_id: 2801, channel: "chat_message_created", payload: "{\"message\" : {\"id\":7,\"chat_id\":1,\"sender_id\":1,\"content\":\"this is a test message\",\"created_at\":\"2024-11-17T00:57:45.398913+00:00\"}, \"members\" : [1,2]}" }
        // { process_id: 2801, channel: "chat_updated", payload: "{\"op\" : \"INSERT\", \"old\" : null, \"new\" : {\"id\":8,\"ws_id\":1,\"name\":\"test chat new b\",\"chat_type\":\"public_channel\",\"members\":[1,4],\"status\":1,\"created_at\":\"2024-11-17T01:01:32.372249+00:00\"}}" }
        let mut stream = listener.into_stream();
        tokio::spawn(async move {
            while let Some(Ok(notif)) = stream.next().await {
                println!(
                    "Received notification: {:?}, current listen user: {:?}",
                    notif,
                    online_users.len()
                );
                let notification = Notification::try_load(notif.channel(), notif.payload())?;
                for user_id in notification.user_ids {
                    if let Some(tx) = online_users.get(&user_id) {
                        info!("Sending notification to user {}", user_id);
                        if let Err(e) = tx.send(notification.event.clone()) {
                            warn!(
                                "Failed to send notification to user {}, error: {}",
                                user_id, e
                            );
                        }
                    }
                }
            }
            Ok::<_, AppError>(())
        });
        Ok(())
    }
}

#[derive(Debug)]
struct Notification {
    user_ids: HashSet<u64>,
    event: Arc<AppEvent>,
}

impl Notification {
    fn try_load(rtype: &str, payload: &str) -> Result<Self, AppError> {
        match rtype {
            "chat_updated" => {
                let payload: ChatUpdated = serde_json::from_str(payload)?;
                let user_ids = payload.get_affected_chat_user_ids();
                let event = match payload.op.as_str() {
                    "INSERT" => AppEvent::NewChat(payload.new.unwrap()),
                    "UPDATE" => AppEvent::AddToChat(payload.new.unwrap()),
                    "DELETE" => AppEvent::RemoveFromChat(payload.old.unwrap()),
                    _ => return Err(AppError::InvalidError(payload.op)),
                };
                Ok(Self {
                    user_ids,
                    event: Arc::new(event),
                })
            }
            "chat_message_created" => {
                let payload: MessageCreated = serde_json::from_str(payload)?;
                let user_ids = payload.members.iter().map(|v| *v as u64).collect();
                Ok(Self {
                    user_ids,
                    event: Arc::new(AppEvent::NewMessage(payload.message)),
                })
            }
            _ => return Err(AppError::InvalidError(rtype.to_owned())),
        }
    }
}
