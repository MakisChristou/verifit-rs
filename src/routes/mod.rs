// My custom routes
mod create_task;
mod get_one_task;
mod hello_world;

use create_task::create_task;
use get_one_task::get_one_task;
use hello_world::hello_world;

use axum::http::Method;
use axum::middleware;
use axum::Extension;
use axum::{
    body::Body,
    routing::{get, post},
    Router,
};
use sea_orm::DatabaseConnection;
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
pub struct SharedData {
    pub message: String,
}

pub async fn create_routes(database: DatabaseConnection) -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let shared_data = SharedData {
        message: String::from("Hello from shared data"),
    };

    Router::new()
        .route("/", get(hello_world))
        .layer(cors)
        .layer(Extension(shared_data))
        .route("/tasks", post(create_task))
        .route("/tasks/:task_id", get(get_one_task))
        .layer(Extension(database))
}
