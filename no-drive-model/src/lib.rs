mod models;
mod schema;


use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use crate::models::{NewUser, User};
use crate::schema::users;

pub fn establish_connection() -> PgConnection {
    dotenv().ok(); // Load environment variables from .env file
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_user(conn: &mut PgConnection, username: &str, password: &str) -> NewUser {
    use diesel::RunQueryDsl;

    let new_user = NewUser {
        username: username.to_string(),
        password: password.to_string(),
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .returning(NewUser::as_returning())
        .get_result(conn)
        .expect("Error saving new user")
}





#[cfg(test)]

#[test]
pub fn db_test() {
    let mut connection = establish_connection();
    let username = String::from("Amah5");
    let password = String::from("123");

    // Create user
    let new_user = create_user(&mut connection, &username, &password);
    assert_eq!(new_user, NewUser { username: "Amah5".to_string(), password: "123".to_string() });


}
