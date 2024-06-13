use std::sync::Arc;
use async_trait::async_trait;
use actix_web::{App, HttpServer, Responder, web, HttpResponse};
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use shaku::{Component, HasComponent, Interface, module};
use tracing_actix_web::TracingLogger;

use no_drive_model::common::app::logger_init;

#[cfg(test)]
mod tests;

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

#[derive(Component)]
#[shaku(interface = IUserService)]
struct UserServiceImpl {
    storage: Arc<std::sync::Mutex<UserStorage>>,
}

#[async_trait]
trait IUserService: Interface {
    async fn register_user(&self, user_data: UserData) -> Result<()>;
    async fn login_user(&self, credentials: Credentials) -> Result<Token>;
}

struct UserStorage {
    users: Vec<UserData>,
}

impl UserStorage {
    fn new() -> Self {
        UserStorage { users: Vec::new() }
    }

    fn add_user(&mut self, user: UserData) -> bool {
        if self.users.iter().any(|u| u.username == user.username) {
            false
        } else {
            self.users.push(user);
            true
        }
    }

    fn find_user(&self, username: &str) -> Option<&UserData> {
        self.users.iter().find(|user| user.username == username)
    }
}

#[async_trait]
impl IUserService for UserServiceImpl {
    async fn register_user(&self, user_data: UserData) -> Result<()> {
        let mut storage = self.storage.lock().unwrap();

        // Perform simple validation
        if user_data.username.is_empty() || user_data.password.is_empty() {
            return Err(anyhow!("Username and password cannot be empty"));
        }

        // Check if the username already exists
        if storage.find_user(&user_data.username).is_some() {
            return Err(anyhow!("Username already taken"));
        }

        // Store the user
        let user_to_add = UserData {
            username: user_data.username.clone(), 
            password: user_data.password, 
        };

        if storage.add_user(user_to_add.clone()) {
            println!("User registered successfully: {:?}", user_to_add);
            Ok(())
        } else {
            Err(anyhow!("Failed to register user"))
        }
    }

    async fn login_user(&self, _credentials: Credentials) -> Result<Token> {
        todo!()
    }
}

module! {
    AppModule {
        components = [UserServiceImpl],
        providers = []
    }
}

async fn register_user(data: web::Data<Arc<AppModule>>, user_data: web::Json<UserData>) -> impl Responder {
    let user_service: &dyn IUserService = data.resolve_ref();

    match user_service.register_user(user_data.into_inner()).await {
        Ok(_) => HttpResponse::Ok().body("User registered"),
        Err(e) => {
            eprintln!("Failed to register user: {}", e);
            HttpResponse::InternalServerError().body("Failed to register user")
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    logger_init().await?;

    let storage = Arc::new(std::sync::Mutex::new(UserStorage::new()));
    let app_module = Arc::new(AppModule::builder()
        .with_component_parameters::<UserServiceImpl>(UserServiceImplParameters {
            storage: storage.clone(),
        })
        .build());

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
