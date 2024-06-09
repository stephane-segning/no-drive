# Technology Stack for Each Service

1. **User Service:**
    - **Web Framework:** Actix Web
    - **OAuth2:** OAuth2 server library like `oxide-auth`
    - **Database:** Diesel ORM or SQLx
    - **Dependency Injection:** `shaku` or `async-trait` for DI
    - **Asynchronous Runtime:** Tokio

2. **File Service:**
    - **Web Framework:** Actix Web
    - **File Storage:** Use standard filesystem operations (`std::fs`) and S3-compatible APIs (e.g., `rusoto`)
    - **Database:** Diesel ORM or SQLx
    - **Dependency Injection:** `shaku` or `async-trait` for DI
    - **Asynchronous Runtime:** Tokio

3. **Notification Service:**
    - **Web Framework:** Actix Web
    - **MQTT:** `rumqttc` for MQTT client/server
    - **Database:** Diesel ORM or SQLx
    - **Dependency Injection:** `shaku` or `async-trait` for DI
    - **Asynchronous Runtime:** Tokio

## Best Practices for Clean Code and DI

1. **Project Structure:**
    - **Separation of Concerns:** Separate code into logical modules (e.g., handlers, services, repositories).
    - **Layered Architecture:** Use layers like controllers, services, and repositories to isolate business logic.
    - **Modules and Crates:** Split the project into multiple crates for better organization and reusability.

2. **Dependency Injection:**
    - Use `shaku` or `async-trait` to implement DI in Rust. This helps decouple components and makes testing easier.
    - Define interfaces (traits) for services and inject implementations at runtime.

3. **Configuration Management:**
    - Use `config` or `dotenv` for managing configuration files and environment variables.

4. **Error Handling:**
    - Use `thiserror` and `anyhow` for robust and user-friendly error handling.
    - Define custom error types and propagate errors using `Result`.

5. **Logging and Monitoring:**
    - Use `tracing` and `tracing-actix-web` for structured logging.
    - Integrate monitoring tools like Prometheus using `prometheus` crate.

6. **Security:**
    - Use `jsonwebtoken` for handling JWT tokens for authentication and authorization.
    - Ensure secure password hashing using `argon2`.