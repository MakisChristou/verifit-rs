use dotenvy::dotenv;
use dotenvy_macro::dotenv;
use verifit_rs::run;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_uri = dotenv!("DATABASE_URL");

    println!("Connecting to : {:?}", database_uri);

    run(database_uri).await;
}
