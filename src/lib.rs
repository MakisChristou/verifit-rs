use sea_orm::Database;
mod routes;
mod database;


use routes::create_routes;


pub async fn run(database_uri: &str) {
    let database = Database::connect(database_uri).await;

    let app = create_routes(database.unwrap()).await;

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

}

