use std::collections::BTreeMap;

use std::{env, println as info};

use axum::{routing::get, Router};
use handlebars::Handlebars;

#[tokio::main]
async fn main() {
    let app = Router::new().route(
        "/chat",
        get(|| async {
            info!("GET /");
            let mut handlebars = Handlebars::new();
            let source = include_str!("templates/index.html");

            if handlebars.register_template_string("t1", source).is_ok() {
                let mut data = BTreeMap::new();
                data.insert("my_var".to_string(), "BBBBBBBBBBBBB".to_string());

                handlebars.render("t1", &data).unwrap()
            } else {
                "Failed to register template".to_string()
            }
        }),
    );

    let port = env::var("PORT").unwrap_or_else(|_| "4002".to_string());
    info!("Chat service starging");
    info!("Server at http://localhost:{port}");

    axum::Server::bind(&format!("0.0.0.0:{port}").parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    info!("Chat service stopped");
}
