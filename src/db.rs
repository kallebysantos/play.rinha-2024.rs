use std::env;

use diesel::{Connection, PgConnection};
use dotenvy::dotenv;

pub fn establish_connection() -> PgConnection {
  dotenv().ok();
  let database_url =
    env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
  PgConnection::establish(&database_url)
    .unwrap_or_else(|_| panic!("Error connecting to {database_url}"))
}
