use std::sync::Arc;

use actix_multipart::form::MultipartForm;
use actix_multipart::form::tempfile::TempFile;
use actix_web::{App, HttpRequest, HttpServer, post, Responder, web};
use async_trait::async_trait;
use log::info;
use minio::s3::client::{Client as MinioClient, ClientBuilder};
use minio::s3::creds::StaticProvider;
use minio::s3::http::BaseUrl;
use shaku::{Component, HasComponent, Interface, module, Module, Provider};
use tokio::sync::Mutex;
use tracing_actix_web::TracingLogger;

use no_drive_model::anyhow::Result;
use no_drive_model::common::app::logger_init;

#[derive(Component)]
#[shaku(interface = IFileService)]
struct MinioFileServiceImpl {
    minio: Arc<Mutex<MinioClient>>,
}

#[derive(Debug, MultipartForm)]
struct FileData {
    #[multipart(rename = "file")]
    file: TempFile,
}

#[async_trait]
impl IFileService for MinioFileServiceImpl {
    async fn upload_file(&self, file_data: TempFile) -> Result<()> {
        let minio_client = self.minio.lock().await;
        todo!("Upload file to Minio")
    }

    async fn download_file(&self, _file_id: String) -> Result<FileData> {
        todo!("Download file from Minio")
    }
}

#[async_trait]
trait IFileService: Interface {
    async fn upload_file(&self, file_data: TempFile) -> Result<()>;
    async fn download_file(&self, file_id: String) -> Result<FileData>;
}

module! {
    AppModule {
        components = [MinioFileServiceImpl],
        providers = []
    }
}

#[post("/upload")]
async fn upload_file(data: web::Data<Arc<AppModule>>, MultipartForm(upload_data): MultipartForm<FileData>, _req: HttpRequest) -> impl Responder {
    info!("Uploading file...");
    let file_service: &dyn IFileService = data.resolve_ref();

    file_service.upload_file(upload_data.file).await.unwrap();
    "File uploaded"
}

#[tokio::main]
async fn main() -> Result<()> {
    logger_init().await?;

    let base_url = "https://play.min.io".parse::<BaseUrl>()?;

    let static_provider = StaticProvider::new(
        "Q3AM3UQ867SPQQA43P2F",
        "zuf+tfteSlswRu7BJ86wekitnifILbZam1KYY3TG",
        None,
    );

    let client = ClientBuilder::new(base_url.clone())
        .provider(Some(Box::new(static_provider)))
        .build()?;

    let module = AppModule::builder()
        .with_component_parameters::<MinioFileServiceImpl>(MinioFileServiceImplParameters {
            minio: Arc::new(Mutex::new(client)), // Wrap MinioClient with Arc and Mutex for thread safety
        })
        .build();
    let app_module = Arc::new(module);

    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .app_data(web::Data::new(app_module.clone()))
            .service(upload_file)
    })
        .bind("0.0.0.0:6000")?
        .run()
        .await?;

    Ok(())
}
