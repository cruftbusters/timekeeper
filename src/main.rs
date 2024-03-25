use std::sync::{Arc, Mutex};
use axum::{
    http::StatusCode,
    Json,
    Router, routing::{get, post},
};
use axum::extract::State;
use maud::{DOCTYPE, html, Markup};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
struct AppState {
    punched_in: Arc<Mutex<bool>>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let state = AppState { punched_in: Arc::new(Mutex::new(false)) };

    let app = Router::new()
        .route("/", get(index))
        .route("/punch", post(punch))
        .route("/heartbeat", get(heartbeat))
        .route("/users", post(create_user))
        .with_state(state);

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

async fn punch(
    State(state): State<AppState>,
) -> Markup {
    let mut punched_in = state.punched_in.lock().expect("mutex was poisoned");
    *punched_in = !*punched_in;
    let text = if *punched_in { "punched in" } else { "punched out" };
    html! { (text) }
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