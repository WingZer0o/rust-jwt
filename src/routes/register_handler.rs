use actix_web::{error, get, web, HttpResponse, Responder};

use crate::models::DbPool;
use crate::datalayer::users_dl;

#[get("/")]
pub async fn hello(pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    
    let user = web::block(move || {
        let mut connection = pool.get().expect("couldn't get db connection from pool");
        let new_user: Result<crate::models::User, ()> = users_dl::insert_new_user(&mut connection);
        new_user
    })
    .await
    .map_err(|e| {
        error::ErrorInternalServerError(e);
    });

    Ok(HttpResponse::Ok().json(user.unwrap().unwrap()))
}