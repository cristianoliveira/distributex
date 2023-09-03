use handlebars::Handlebars;
use serde_json::json;

use crate::{http_handlers::ChatUser, ws_handlers::Event};

// load templates for chat page using handlerbars
pub fn index_page() -> String {
    let mut reg = Handlebars::new();

    reg.register_template_string("index", include_str!("templates/index.html"))
        .unwrap();
    reg.register_partial("join_form", include_str!("templates/_join_form.html"))
        .unwrap();

    reg.render("index", &json!({})).unwrap()
}

pub fn chat(user: ChatUser) -> String {
    let mut reg = Handlebars::new();

    reg.register_template_string("chat", include_str!("templates/chat.html"))
        .unwrap();

    reg.render("chat", &json!(user)).unwrap()
}

pub fn chat_message(event: Event) -> String {
    let mut reg = Handlebars::new();

    reg.register_template_string("chat_message", include_str!("templates/message.html"))
        .unwrap();

    reg.render("chat_message", &json!(event)).unwrap()
}

pub fn chat_join(username: &String) -> String {
    let mut reg = Handlebars::new();

    reg.register_template_string("chat_input", include_str!("templates/chat_input.html"))
        .unwrap();

    reg.render("chat_input", &json!({"username": username}))
        .unwrap()
}
