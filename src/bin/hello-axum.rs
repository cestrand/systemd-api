use axum::{
    http::StatusCode, response::Redirect, routing::{get, post}, Json, Router
};
use serde::{Deserialize, Serialize};
use tracing::info;
use utoipa::{OpenApi};

mod systemd_version;

#[derive(OpenApi)]
#[openapi(paths(systemd_version::get::handler))]
struct ApiDoc;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
    .with_max_level(tracing::Level::INFO)
    .init();

    println!("{}", ApiDoc::openapi().to_pretty_json().unwrap());

    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user))
        .route("/version", get(systemd_version::get::handler))
        .fallback(anything_else);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3123").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    info!("GET root");
    "Hello, World!"
}

async fn create_user(
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    info!("POST create_user");

    let user = User {
        id: 1337,
        username: payload.username,
    };

    (StatusCode::CREATED, Json(user))
}

async fn anything_else() -> Redirect {
    Redirect::to("/")
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