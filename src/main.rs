use axum::{routing::get, Router};
use maud::{html, Markup};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(index));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

async fn index() -> Markup {
    html! {
        h1 { "hello, world!" }
    }
}
