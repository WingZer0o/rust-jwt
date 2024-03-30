use std::time::SystemTime;
use diesel::{prelude::*, r2d2::{self, ConnectionManager}};
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: uuid::Uuid,
    pub email: String,
    pub hash: String,
    pub created_at: SystemTime,
}