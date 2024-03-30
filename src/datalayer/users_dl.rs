use std::time::SystemTime;
use diesel::PgConnection;
use diesel::prelude::*;
use crate::{models::User, schema};

pub fn insert_new_user(connection: &mut PgConnection) -> Result<User, ()> {
    let uuid = uuid::Uuid::new_v4();
    let new_user = User  {
        id: uuid,
        email: "mikemulchrone987@gmail.com".to_owned(),
        created_at: SystemTime::now()
    };

    diesel::insert_into(schema::users::table)
        .values(&new_user)
        .execute(connection)
        .expect("Error inserting user");

    Ok(new_user)
}