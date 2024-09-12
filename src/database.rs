use crate::configuration::Settings;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

pub async fn _get_database(configuration: &Settings) {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&configuration.database.url)
        .await
        .expect("Failed to connect to the database");
}
