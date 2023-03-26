// My custom routes
mod create_exercise;
mod create_workout_set;
mod delete_exercise;
mod delete_set;
mod get_exercises;
mod get_workout_sets;
mod guard;
mod hello_world;
mod update_exercises;
mod update_sets;
mod users;

use axum::routing::delete;
use axum::routing::put;
use create_exercise::create_exercise;
use create_workout_set::{create_workout_set, create_workout_sets};
use delete_exercise::delete_exercise;
use delete_set::{delete_set, delete_sets};
use get_exercises::{get_all_exercises, get_one_exercise};
use get_workout_sets::{get_all_workout_sets, get_one_workout_set};
use guard::guard;
use hello_world::{hello_world, privacy_policy};
use update_exercises::atomic_update_exercise;
use update_sets::{atomic_update_set, atomic_update_sets};
use users::{
    change_password, create_user, login, logout, request_email_verification,
    request_password_reset, verify_email,
};

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
        .route("/users/logout", post(logout))
        // .route("/exercises", post(create_exercise))
        .route("/sets", post(create_workout_set))
        .route("/sets/bulk", post(create_workout_sets))
        .route("/sets/bulk", delete(delete_sets))
        .route("/sets/bulk", put(atomic_update_sets))
        .route("/sets", get(get_all_workout_sets))
        .route("/sets/:set_id", get(get_one_workout_set))
        .route("/sets/:set_id", delete(delete_set))
        .route("/sets/:set_id", put(atomic_update_set))
        .route_layer(middleware::from_fn(guard))
        .route(
            "/users/request-password-reset",
            post(request_password_reset),
        )
        .route(
            "/users/request-email-verification",
            post(request_email_verification),
        )
        .route("/users/verify-email", get(verify_email))
        .route("/users/change-password", post(change_password))
        .route("/", get(hello_world))
        .route("/privacy_policy", get(privacy_policy))
        .layer(cors)
        .layer(Extension(shared_data))
        .route("/users", post(create_user))
        .route("/users/login", post(login))
        .layer(Extension(database))
}
