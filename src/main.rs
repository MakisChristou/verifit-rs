use dotenvy_macro::dotenv;
use dotenvy::dotenv;
use verifit_database::run;

#[tokio::main]
async fn main() {

    dotenv().ok();
    let database_uri = dotenv!("DATABASE_URL");

    run(database_uri).await;
}
