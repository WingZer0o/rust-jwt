use std::env::var;
use dotenvy::dotenv;

pub fn database_url() -> String {
    dotenv().ok();
    var("DATABASE_URL").expect("DATABASE_URL is not set")
}