use axum::{
    extract::{Query, Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use axum_extra::{headers::authorization::Bearer, TypedHeader};
use serde_json::json;

use crate::{common::AuthInfo, error::ErrorOutput, AppState};

pub async fn verify_token(
    State(state): State<AppState>,
    bearer: Option<TypedHeader<axum_extra::headers::Authorization<Bearer>>>,
    query: Option<Query<AuthInfo>>,
    mut req: Request,
    next: Next,
) -> Response {
    let token = match (&bearer, &query) {
        (Some(TypedHeader(bearer)), _) => bearer.token(),
        (_, Some(Query(AuthInfo { ref token }))) => token,
        _ => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!(ErrorOutput::new("need token"))),
            )
                .into_response()
        }
    };
    match state.auth.verify_token(token) {
        Ok(user) => req.extensions_mut().insert(user),
        Err(e) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!(ErrorOutput::new(format!(
                    "parse Authorization header failed: {:?}",
                    e
                )))),
            )
                .into_response();
        }
    };
    next.run(req).await
}
