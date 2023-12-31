use axum::{
    extract::Form,
    response::{Html, IntoResponse},
};
use serde::{Deserialize, Serialize};

use crate::templates;

#[derive(Deserialize, Serialize, Debug)]
pub struct ChatUser {
    username: String,
}

pub async fn join(Form(params): Form<ChatUser>) -> impl IntoResponse {
    Html(templates::chat(params))
}

pub async fn chat_app() -> impl IntoResponse {
    Html(templates::chat_app())
}

pub async fn index_page() -> impl IntoResponse {
    Html(templates::index_page())
}
