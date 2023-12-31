mod http_handlers;
mod templates;
mod ws_handlers;

use log::info;

use axum::{
    routing::get,
    Router
};
use std::{
    collections::HashSet,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tokio::sync::broadcast;

pub struct AppState {
    // Set of connected usernames.
    user_set: Mutex<HashSet<String>>,
    // Channel used to broadcast messages to the websocket clients.
    tx: broadcast::Sender<String>,
}

#[tokio::main]
async fn main() {
    info!("Starting chat server...");
    let port = std::env::var("PORT").unwrap_or_else(|_| "4002".to_string());
    let user_set = Mutex::new(HashSet::new());
    let (tx, _rx) = broadcast::channel(100);

    let app_state = Arc::new(AppState { user_set, tx });

    let app = Router::new()
        .route("/", get(crate::http_handlers::index_page))
        .route("/chat", get(crate::http_handlers::chat_app))
        .route("/chat/join", get(crate::http_handlers::join))
        .route("/chat/ws", get(crate::ws_handlers::chat))
        .with_state(app_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], port.parse::<u16>().unwrap()));
    info!("Listening {}.", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
