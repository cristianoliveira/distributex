use crate::templates;
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::AppState;

#[derive(Deserialize, Serialize, Debug)]
pub struct Event {
    username: Option<String>,
    message: String,
}

pub async fn chat(ws: WebSocketUpgrade, State(state): State<Arc<AppState>>) -> impl IntoResponse {
    ws.on_upgrade(|socket| websocket(socket, state))
}

async fn websocket(stream: WebSocket, state: Arc<AppState>) {
    let (mut sender, mut receiver) = stream.split();

    let mut username = String::new();
    while let Some(Ok(message)) = receiver.next().await {
        if let Message::Text(raw_text) = message {
            println!("Received message: {:?}", raw_text);
            let data = match serde_json::from_str::<Event>(&raw_text) {
                Ok(event) => event,
                Err(err) => {
                    println!("Invalid JSON: {:?}", err);
                    continue;
                }
            };

            if data.username.is_none() {
                println!("Invalid username: {:?}", raw_text);
                continue;
            }

            println!("Username: {:?}", data);
            let name = data.username.unwrap();
            check_username(&state, &mut username, &name);

            if !username.is_empty() {
                break;
            } else {
                let _ = sender
                    .send(Message::Text(String::from("Username already taken.")))
                    .await;

                return;
            }
        }
    }

    let mut rx = state.tx.subscribe();
    let msg = format!("{} joined.", username);
    let _ = state.tx.send(msg);

    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            println!("Sending message: {:?}", msg);
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    let tx = state.tx.clone();
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(raw_text))) = receiver.next().await {
            let data = match serde_json::from_str::<Event>(&raw_text) {
                Ok(event) => event,
                Err(err) => {
                    println!("Invalid JSON: {:?}", err);
                    continue;
                }
            };

            if data.username.is_none() {
                println!("Invalid username: {:?}", raw_text);
                continue;
            }

            let response_html = templates::chat_message(Event {
                username: data.username,
                message: data.message,
            });

            let res = tx.send(response_html);
            println!("Res after send msg: {:?}", res);
        }
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };

    let msg = format!("{} left.", username);
    let _ = state.tx.send(msg);

    state.user_set.lock().unwrap().remove(&username);
}

fn check_username(state: &AppState, string: &mut String, name: &str) {
    let mut user_set = state.user_set.lock().unwrap();

    if !user_set.contains(name) {
        user_set.insert(name.to_owned());

        string.push_str(name);
    }
}
