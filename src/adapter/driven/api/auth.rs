use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

use crate::{
    error::AppError,
    service::auth::{SigninUserDto, SignupUserDto},
    AppState,
};

#[derive(Serialize)]
struct TokenDto {
    token: String,
}
impl TokenDto {
    fn new(token: String) -> Self {
        Self { token }
    }
}
pub async fn signin_handler(
    State(state): State<AppState>,
    Json(input): Json<SigninUserDto>,
) -> Result<impl IntoResponse, AppError> {
    let token = state.auth.signin(input).await?;
    Ok((StatusCode::OK, Json(TokenDto::new(token))))
}

pub async fn signup_handler(
    State(state): State<AppState>,
    Json(input): Json<SignupUserDto>,
) -> Result<impl IntoResponse, AppError> {
    let token = state.auth.signup(input).await?;
    Ok((StatusCode::CREATED, Json(TokenDto::new(token))))
}
