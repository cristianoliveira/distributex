use handlebars::Handlebars;
use serde_json::json;

use crate::ws_handlers::Event;

// load templates for chat page using handlerbars
pub fn index_page(username: String) -> String {
    let mut reg = Handlebars::new();

    reg.register_template_string("index", include_str!("templates/index.html"))
        .unwrap();

    reg.render("index", &json!({"username": username})).unwrap()
}

pub fn chat_message(event: Event) -> String {
    let mut reg = Handlebars::new();

    reg.register_template_string("chat_message", include_str!("templates/message.html"))
        .unwrap();

    reg.render("chat_message", &json!(event)).unwrap()
}
