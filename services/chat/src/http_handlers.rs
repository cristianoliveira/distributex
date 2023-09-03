use axum::{
    extract::{Query, State},
    response::{Html, IntoResponse},
};
use serde::Deserialize;
use std::{sync::Arc, time::SystemTime};

use crate::{templates, AppState};

#[derive(Deserialize, Debug)]
pub struct ChatUser {
    username: Option<String>,
}

pub async fn index_page(
    Query(query): Query<ChatUser>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let user_set = state.user_set.lock().unwrap();
    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);
    let username = if let Some(value) = query.username {
        if !user_set.contains(&value) {
            // user_set.insert(value.to_owned());
            value
        } else {
            format!("-{}", now.unwrap().as_millis())
        }
    } else {
        format!("-{}", now.unwrap().as_millis())
    };

    println!("User set: {:?}", username);
    Html(templates::index_page(username))
}
