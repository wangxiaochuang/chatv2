use crate::{error::AppError, service::auth::ClaimUser, AppState};
use anyhow::anyhow;
use axum::{
    extract::{Path, Request, State},
    middleware::Next,
    response::{IntoResponse as _, Response},
    Extension,
};

pub async fn check_msg_perm(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Extension(user): Extension<ClaimUser>,
    req: Request,
    next: Next,
) -> Response {
    // check user in chat and chat belongs to ws
    match state.chat.get_by_id(id).await {
        Ok(Some(chat)) => {
            if chat.status == 0
                || user.ws_id as i64 != chat.ws_id
                || !chat.members.contains(&(user.id as i64))
            {
                return AppError::NotFound("chat not found".to_owned()).into_response();
            }
        }
        Ok(None) => return AppError::NotFound("chat not exist".to_owned()).into_response(),
        Err(_) => return AppError::AnyError(anyhow!("system error")).into_response(),
    };
    next.run(req).await
}
