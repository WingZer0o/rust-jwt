use actix_web::{error, post, web, HttpResponse, Responder};
use serde::Deserialize;

use crate::models::DbPool;
use crate::datalayer::users_dl;

#[derive(Deserialize)]
pub struct RegisterUser {
    email: String,
}

#[post("/register-user")]
pub async fn register_user(body: web::Json<RegisterUser>, pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    let user = web::block(move || {
        let mut connection = pool.get().expect("couldn't get db connection from pool");
        let new_user: Result<crate::models::User, ()> = users_dl::insert_new_user(body.email.to_string(), &mut connection);
        new_user
    })
    .await
    .map_err(|e| {
        error::ErrorInternalServerError(e);
    });

    Ok(HttpResponse::Ok().json(user.unwrap().unwrap()))
}