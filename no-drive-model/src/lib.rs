pub use anyhow;
pub use shaku;
pub use thiserror;

mod models;
mod schema;
mod common;

use diesel::r2d2::{self, ConnectionManager};
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use anyhow::Context;
use common::user_service::{IUserService, UserService};
use shaku::HasComponent;
use crate::models::NewUser;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[shaku::module]
module! {
    AppModule {
        components = [UserService],
        providers = []
    }
}

pub fn establish_connection_pool() -> Result<Pool, anyhow::Error> {
    dotenv().ok(); // Load environment variables from .env file
    let database_url = env::var("DATABASE_URL").context("DATABASE_URL must be set in .env file")?;
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager).context("Failed to create pool")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    struct AppModule();

    #[test]
    pub fn db_test() -> Result<(), Box<dyn std::error::Error>> {
        let pool = establish_connection_pool()?;
        let module = AppModule::builder()
            .with_component_override::<dyn IUserService>(Box::new(UserService {
                pool: Arc::new(pool),
            }))
            .build();

        let user_service: &dyn IUserService = module.resolve_ref();
        let username = "Amah5";
        let password = "123";

        // Create user
        let new_user = user_service.create_user(username, password)?;
        assert_eq!(new_user, NewUser { username: username.to_string(), password: password.to_string() });

        Ok(())
    }
}
