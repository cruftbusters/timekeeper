use axum::{
    http::StatusCode,
    Json,
    Router, routing::{get, post},
};
use maud::{DOCTYPE, html, Markup};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(index))
        .route("/punch", post(punch))
        .route("/heartbeat", get(heartbeat))
        .route("/users", post(create_user));

    let address = "[::]:3000";
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn index() -> Markup {
    html! {
        (DOCTYPE)
        html {
            script src="https://unpkg.com/htmx.org@1.9.11" {}
            button hx-post="/punch" hx-swap="innerHTML" { "punched out" }
        }
    }
}

async fn punch() -> Markup {
    html! { "punched in" }
}

async fn heartbeat() -> Markup {
    html! {
        (DOCTYPE)
        html {
           "Hello, World!"
        }
    }
}

async fn create_user(
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    let user = User {
        id: 1337,
        username: payload.username,
    };

    (StatusCode::CREATED, Json(user))
}

#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}