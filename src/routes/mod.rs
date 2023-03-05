
// My custom routes
mod hello_world;
use hello_world::hello_world;


use axum::middleware;
use axum::http::Method;
use axum::Extension;
use axum::{
    body::Body,
    routing::{get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
pub struct SharedData {
    pub message: String,
}

pub fn create_routes() -> Router {
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
}
