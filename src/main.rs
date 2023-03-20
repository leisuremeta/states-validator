use std::time::Duration;

use log::LevelFilter;
use sea_orm::{DatabaseConnection, ConnectOptions, Database};



async fn db_connn(database_url: String) -> DatabaseConnection {
  let mut opt = ConnectOptions::new(database_url.to_string());
  opt.min_connections(1)
     .max_connections(2)
     .connect_timeout(Duration::from_secs(30))
     .acquire_timeout(Duration::from_secs(30))
     .idle_timeout(Duration::from_secs(120))
     .set_schema_search_path("public".into())
     .sqlx_logging(true)
     .sqlx_logging_level(LevelFilter::Debug);

  match Database::connect(opt).await {
    Ok(conn) => conn,
    Err(err) =>  panic!("{err}"),
  }
}

async fn validate_account_balances(db: &DatabaseConnection) {
  
}



#[tokio::main]
async fn main() {
  dotenvy::dotenv().expect("Unable to load environment variables from .env file");

  let database_url = dotenvy::var("DATABASE_URL").expect("DATABASE_URL must be set.");

  let db = db_connn(database_url).await;

  validate_account_balances(&db).await;
}

