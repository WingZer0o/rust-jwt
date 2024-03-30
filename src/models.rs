use std::time::SystemTime;
use diesel::{prelude::*, r2d2::{self, ConnectionManager}};
use serde::{Deserialize, Serialize};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: uuid::Uuid,
    pub email: String,
    pub created_at: SystemTime,
}