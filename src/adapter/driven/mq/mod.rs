use futures::StreamExt as _;
use sqlx::postgres::PgListener;

use crate::{error::AppError, AppState};

pub async fn monitor_mq_change(state: AppState) -> Result<(), AppError> {
    let mut listener = PgListener::connect(&state.config.server.db_url).await?;
    listener.listen("chat_updated").await?;
    listener.listen("chat_message_created").await?;

    let mut stream = listener.into_stream();
    tokio::spawn(async move {
        while let Some(Ok(notif)) = stream.next().await {
            println!(
                "Received notification: {:?}, current listen user: {:?}",
                notif, 1
            );
        }
    });
    Ok(())
}
