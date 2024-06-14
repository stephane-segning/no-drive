use diesel::{Insertable, Queryable, Selectable};
use crate::schema::users;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(PartialEq, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Insertable, Queryable, Selectable)]
#[diesel(table_name = users)]
#[derive(PartialEq, Debug)]
pub struct NewUser {
    pub username: String,
    pub password: String,
}
