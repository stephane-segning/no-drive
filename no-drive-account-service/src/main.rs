use std::sync::Arc;
use async_trait::async_trait;
use actix_web::{App, HttpServer, Responder, web};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use shaku::{Component, HasComponent, Interface, module};
use tracing_actix_web::TracingLogger;

use no_drive_model::common::app::logger_init;

#[cfg(test)]
mod tests;

#[derive(Component)]
#[shaku(interface = IUserService)]
struct UserServiceImpl;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
struct UserData {
    username: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
struct Credentials {
    username: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
struct Token(String);
#[async_trait]
impl IUserService for UserServiceImpl {
    async fn register_user(&self, user_data: UserData) -> Result<()> {
        todo!()
    }

   async fn login_user(&self, credentials: Credentials) -> Result<Token> {
        todo!()
    }
}
#[async_trait]
trait IUserService: Interface {
    async fn register_user(&self, user_data: UserData) -> Result<()>;
    async fn login_user(&self, credentials: Credentials) -> Result<Token>;
}

module! {
    AppModule {
        components = [UserServiceImpl],
        providers = []
    }
}

async fn register_user(data: web::Data<Arc<AppModule>>, user_data: web::Json<UserData>) -> impl Responder {
    let user_service: &dyn IUserService = data.resolve_ref();
    user_service.register_user(user_data.into_inner()).await.unwrap();
    "User registered"
}

#[tokio::main]
async fn main() -> Result<()> {
    logger_init().await?;

    let app_module = Arc::new(AppModule::builder().build());

    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .app_data(web::Data::new(app_module.clone()))
            .route("/register", web::post().to(register_user))
    })
        .bind("0.0.0.0:6001")?
        .run()
        .await?;

    Ok(())
}
