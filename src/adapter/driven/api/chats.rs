use std::{convert::Infallible, time::Duration};

use crate::{
    error::AppError,
    service::{
        auth::ClaimUser,
        chat::{CreateChatDto, ListOptionsDto, SendMsgDto, UpdateChatDto},
    },
    AppState,
};

use axum::{
    extract::{Multipart, Path, Query, State},
    http::StatusCode,
    response::{
        sse::{Event, KeepAlive},
        IntoResponse, Sse,
    },
    Extension, Json,
};
use futures::Stream;
use tokio_stream::{wrappers::BroadcastStream, StreamExt};
pub async fn get(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<impl IntoResponse, AppError> {
    let chat = state.chat.get_by_id(id).await?;
    Ok((StatusCode::CREATED, Json(chat)))
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(input): Json<UpdateChatDto>,
) -> Result<impl IntoResponse, AppError> {
    let msg = state.chat.update(id, input).await?;
    Ok((StatusCode::CREATED, Json(msg)))
}

pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<impl IntoResponse, AppError> {
    let chat = state.chat.delete(id).await?;
    Ok((StatusCode::OK, Json(chat)))
}

pub async fn list_all(
    State(state): State<AppState>,
    Extension(user): Extension<ClaimUser>,
) -> Result<impl IntoResponse, AppError> {
    state.chat.list_all(user.id, user.ws_id).await.map(Json)
}

pub async fn create(
    State(state): State<AppState>,
    Extension(user): Extension<ClaimUser>,
    Json(input): Json<CreateChatDto>,
) -> Result<impl IntoResponse, AppError> {
    let chat = state.chat.create(user.id, user.ws_id, input).await?;
    Ok((StatusCode::CREATED, Json(chat)))
}

pub async fn send_msg(
    State(state): State<AppState>,
    Extension(user): Extension<ClaimUser>,
    Path(id): Path<u64>,
    Json(input): Json<SendMsgDto>,
) -> Result<impl IntoResponse, AppError> {
    let msg = state.chat.send_msg(&user, id, input).await?;
    Ok((StatusCode::CREATED, Json(msg)))
}

pub async fn list_messages(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Query(input): Query<ListOptionsDto>,
) -> Result<impl IntoResponse, AppError> {
    let last_id = input.last_id.unwrap_or(i64::MAX as _);
    let limit = input.limit.unwrap_or(10);
    let msgs = state.chat.list_messages(id, last_id, limit as _).await?;
    Ok((StatusCode::OK, Json(msgs)))
}

pub async fn upload(
    State(state): State<AppState>,
    Extension(user): Extension<ClaimUser>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, AppError> {
    let mut urls = vec![];
    while let Some(field) = multipart.next_field().await.unwrap() {
        if let Some(url) = state.file.save(user.ws_id, field).await? {
            urls.push(url);
        }
    }
    Ok((StatusCode::CREATED, Json(urls)))
}

pub async fn notif_handler(
    State(state): State<AppState>,
    Extension(user): Extension<ClaimUser>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let rx = state.notif.register(user.id);
    let stream = BroadcastStream::new(rx)
        .filter_map(|v| v.ok())
        .map(|ref v| {
            let name = v.get_name();
            let data = serde_json::to_string(v).unwrap();
            Ok(Event::default().data(data).event(name))
        });
    Sse::new(stream).keep_alive(
        KeepAlive::new()
            .interval(Duration::from_secs(30))
            .text("alive"),
    )
}
