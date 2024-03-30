use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use diesel::{r2d2::{self, ConnectionManager}, PgConnection};
use dotenvy::dotenv;


mod vars;
mod schema;
mod models;
mod routes;
mod datalayer;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let manager = ConnectionManager::<PgConnection>::new(vars::database_url());
    let pool: models::DbPool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create a database connection pool.");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(routes::register_handler::hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}