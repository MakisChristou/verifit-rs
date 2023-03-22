use sea_orm::Database;
mod database;
mod routes;
mod utils;
use log::{info, warn};

use routes::create_routes;

pub async fn run(database_uri: &str) {
    let database = Database::connect(database_uri).await;

    let app = create_routes(database.unwrap()).await;
    let bind_ip = String::from("0.0.0.0:3001");
    warn!("Server started at {}", bind_ip);

    axum::Server::bind(&bind_ip.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
