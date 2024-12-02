use axum::{extract::State, response::IntoResponse, Extension, Json};

use crate::{error::AppError, service::auth::ClaimUser, AppState};

pub async fn list_all_users(
    State(state): State<AppState>,
    Extension(user): Extension<ClaimUser>,
) -> Result<impl IntoResponse, AppError> {
    state.user.fetch_all_in_ws(user.ws_id).await.map(Json)
}

pub async fn get_user_info(
    State(state): State<AppState>,
    Extension(user): Extension<ClaimUser>,
) -> Result<impl IntoResponse, AppError> {
    match state.user.get_user_info(user.id).await? {
        Some(info) => Ok(Json(info)),
        None => Err(AppError::NotFound(user.id.to_string())),
    }
}
