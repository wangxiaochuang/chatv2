use anyhow::Result;
use chat::{build_http_router, init_log, AppState};
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    let state = AppState::try_new_from_conf().await.unwrap();
    init_log(&state);

    let addr = state.config.server.local_addr();
    info!("server listening on {}", addr);

    let listener = TcpListener::bind(&addr).await?;
    axum::serve(listener, build_http_router(&state).into_make_service()).await?;
    Ok(())
}
