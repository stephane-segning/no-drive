use std::sync::Arc;
use actix_web::{App, HttpServer, Responder, web};
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use shaku::{Component, Interface, module};
use tracing_actix_web::TracingLogger;
use shaku::HasComponent; // Add import for HasComponent trait

// Import logging initialization function from your application
use no_drive_model::common::app::logger_init;

#[cfg(test)]
mod tests;

// Define your domain structs
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

// Define the UserService component using Shaku
#[derive(Component)]
#[shaku(interface = IUserService)]
struct UserServiceImpl;

// Define the IUserService trait for dependency injection
trait IUserService: Interface {
    fn register_user(&self, user_data: UserData) -> Result<()>;
    fn login_user(&self, credentials: Credentials) -> Result<Token>;
}

// Mock user storage
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

// Implement IUserService for UserServiceImpl
impl IUserService for UserServiceImpl {
    fn register_user(&self, user_data: UserData) -> Result<()> {
        let mut storage = UserStorage::new();

        // Perform simple validation
        if user_data.username.is_empty() || user_data.password.is_empty() {
            return Err(anyhow!("Username and password cannot be empty"));
        }

        // Check if the username already exists
        if storage.find_user(&user_data.username).is_some() {
            return Err(anyhow!("Username already taken"));
        }

        // Simulate hashing the password (in a real application, use a proper hashing library)
        let hashed_password = format!("hashed_{}", user_data.password);

        // Store the user
        let user_to_add = UserData {
            username: user_data.username.clone(), // Clone username for logging
            password: hashed_password,
        };

        if storage.add_user(user_to_add.clone()) {
            println!("User registered successfully: {:?}", user_to_add);
            Ok(())
        } else {
            Err(anyhow!("Failed to register user"))
        }
    }

    fn login_user(&self, _credentials: Credentials) -> Result<Token> {
        todo!()
    }
}

// Define the AppModule module for dependency injection
module! {
    AppModule {
        components = [UserServiceImpl],
        providers = []
    }
}

// Handler for registering a user
async fn register_user(data: web::Data<Arc<AppModule>>, user_data: web::Json<UserData>) -> impl Responder {
    let app_module = data.get_ref();
    let user_service = app_module.resolve_ref();

    
    match user_service.register_user(user_data.into_inner()) {
        Ok(_) => "User registered",
        Err(e) => {
            eprintln!("Failed to register user: {}", e);
            "Failed to register user"
        }
    }
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
