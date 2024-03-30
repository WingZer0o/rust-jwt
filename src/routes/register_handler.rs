use actix_web::{get, HttpResponse, Responder};

use crate::models::User;

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}