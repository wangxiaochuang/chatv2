mod state;
use adapter::driven::api::{
    chats,
    middlewares::{check_msg_perm, verify_token},
    users,
};
use axum::{
    middleware::from_fn_with_state,
    routing::{get, post},
    Router,
};
pub use state::AppState;
mod adapter;
mod common;
mod domain;
mod error;
mod log;
mod service;
pub use adapter::driven::api::auth;

pub async fn index_handler() -> &'static str {
    "index"
}

pub fn build_http_router(state: &AppState) -> Router {
    let chat = Router::new()
        .route(
            "/:id",
            get(chats::get)
                .patch(chats::update)
                .delete(chats::delete)
                .post(chats::send_msg),
        )
        .route("/:id/messages", get(chats::list_messages))
        .layer(from_fn_with_state(state.clone(), check_msg_perm))
        .route("/", get(chats::list_all).post(chats::create));
    let api = Router::new()
        .route("/users", get(users::list_all_users))
        .route("/userinfo", get(users::get_user_info))
        .nest("/chats", chat)
        .route("/upload", post(chats::upload))
        .route("/events", post(chats::notif_handler))
        .layer(from_fn_with_state(state.clone(), verify_token))
        .route("/signin", post(auth::signin_handler))
        .route("/signup", post(auth::signup_handler));
    Router::new()
        .route("/", get(index_handler))
        .nest("/api", api)
        .with_state(state.clone())
}

pub fn init_log(state: &AppState) {
    log::init(state);
}
