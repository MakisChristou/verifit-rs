use dotenvy::dotenv;
use dotenvy_macro::dotenv;
use log::{info, warn};
use simplelog::{Config, ConfigBuilder, LevelFilter, WriteLogger};
use std::fs::{File, OpenOptions};
use verifit_rs::run;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_uri = dotenv!("DATABASE_URL");

    // Set up simplelog to write logs to a file and only log messages from your own module.
    let log_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("/var/log/verifit-rs/verifit-rs.log")
        .expect("Unable to create or open log file");

    WriteLogger::init(LevelFilter::Warn, Config::default(), log_file)
        .expect("Unable to initialize logger");

    // Log a message.
    warn!("Starting the server");
    warn!("Connecting to {:?}", database_uri);

    run(database_uri).await;
}
