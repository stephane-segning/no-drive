use std::sync::Arc;

use actix_web::{App, HttpServer, Responder, web};
use anyhow::Result;
use async_trait::async_trait;
use rumqttc::{AsyncClient, MqttOptions};
use shaku::{Component, HasComponent, Interface, module, Module, Provider};
use thiserror::Error;
use tracing_actix_web::TracingLogger;

use no_drive_model::common::app::logger_init;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("DB Error")]
    DbError(#[from] diesel::result::Error),

    #[error("unknown error")]
    Unknown,
}

#[derive(Component)]
#[shaku(interface = INotificationService)]
struct NotificationServiceImpl {
    mqtt_client: AsyncClient,
}
#[async_trait]
impl INotificationService for NotificationServiceImpl {
    async fn subscribe(&self, user_id: String, topics: Vec<String>) -> Result<()> {
        todo!()
    }

    async fn send_notification(&self, user_id: String, message: String) -> Result<()> {
        todo!()
    }
}
#[async_trait]
trait INotificationService: Interface {
    async fn subscribe(&self, user_id: String, topics: Vec<String>) -> Result<()>;
    async fn send_notification(&self, user_id: String, message: String) -> Result<()>;
}

module! {
    AppModule {
        components = [NotificationServiceImpl],
        providers = [AsyncClient]
    }
}

impl Provider<AppModule> for AsyncClient {
    type Interface = AsyncClient;

    fn provide(module: &AppModule) -> Result<Box<AsyncClient>, Box<(dyn std::error::Error + 'static)>> {
        let mqtt_options = MqttOptions::new("client_id", "broker.hivemq.com", 1883);
        let (mqtt_client, _mqtt_event_loop) = AsyncClient::new(mqtt_options, 10);
        Ok(Box::new(mqtt_client))
    }
}

async fn subscribe(data: web::Data<Arc<AppModule>>, user_id: web::Path<String>, topics: web::Json<Vec<String>>) -> impl Responder {
    let notification_service: &dyn INotificationService = data.resolve_ref();
    notification_service.subscribe(user_id.into_inner(), topics.into_inner()).await.unwrap();
    "Subscribed"
}

#[tokio::main]
async fn main() -> Result<()> {
    logger_init().await?;

    let app_module = Arc::new(AppModule::builder().build());

    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .app_data(web::Data::new(app_module.clone()))
            .route("/subscribe/{user_id}", web::post().to(subscribe))
    })
        .bind("0.0.0.0:6002")?
        .run()
        .await?;

    Ok(())
}
