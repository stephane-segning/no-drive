use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager, PooledConnection};
use diesel::prelude::*;
use shaku::{Component, Interface};
use crate::models::{NewUser, User};
use crate::schema::users;
use anyhow::Result;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub trait IUserService: Interface {
    fn create_user(&self, username: &str, password: &str) -> Result<NewUser>;
    fn get_user(&self, id: i32) -> Result<Option<User>>;
}

#[derive(Component)]
#[shaku(interface = IUserService)]
pub struct UserService {
    #[shaku(inject)]
    pub(crate) pool: std::sync::Arc<Pool>,
}

impl UserService {
    fn get_conn(&self) -> Result<PooledConnection<ConnectionManager<PgConnection>>> {
        Ok(self.pool.get()?)
    }
}

impl IUserService for UserService {
    fn create_user(&self, username: &str, password: &str) -> Result<NewUser> {
        let conn = &mut self.get_conn()?;
        let new_user = NewUser {
            username: username.to_string(),
            password: password.to_string(),
        };

        diesel::insert_into(users::table)
            .values(&new_user)
            .returning((users::username, users::password)) // Assuming you want to return the username and password
            .get_result(conn)
            .map_err(|e| e.into())
    }

    fn get_user(&self, id: i32) -> Result<Option<User>> {
        let conn = &mut self.get_conn()?;
        users::table.find(id).first(conn).optional().map_err(|e| e.into())
    }
}
