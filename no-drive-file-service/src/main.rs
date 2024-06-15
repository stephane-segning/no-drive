use std::path::PathBuf;
use std::sync::Arc;
use actix_multipart::form::MultipartForm;
use actix_multipart::form::tempfile::TempFile;
use actix_web::{App, HttpServer, post, Responder, web};
use async_trait::async_trait;
use log::info;
use shaku::{Component, HasComponent, Interface, module, Module};
use tracing_actix_web::TracingLogger;
use no_drive_model::anyhow::Result;
use no_drive_model::common::app::logger_init;

#[derive(Component)]
#[shaku(interface = IFileService)]
struct LocalFileServiceImpl {
    #[shaku(default = String::from("./tmp"))]
    folder_path: PathBuf,
}

#[derive(Debug, MultipartForm)]
struct FileData {
    #[multipart(rename = "file")]
    file: TempFile,
}

#[async_trait]
impl IFileService for LocalFileServiceImpl {
    async fn upload_file(&mut self, file_data: TempFile) -> Result<()> {
        todo!("Upload file to local_fs")
    }
    async fn download_file(&self, _file_id: String) -> Result<FileData> {
        todo!("Download file from local_fs")
    }
}

#[async_trait]
trait IFileService: Interface {
    async fn upload_file(&self, file_data: TempFile) -> Result<()>;
    async fn download_file(&self, file_id: String) -> Result<FileData>;
}

module! {
    AppModule {
        components = [LocalFileServiceImpl],
        providers = []
    }
}

#[post("/upload")]
async fn upload_file(data: web::Data<Arc<AppModule>>, MultipartForm(upload_data): MultipartForm<FileData>) -> impl Responder {
    info!("Uploading file...");
    let file_service: &dyn IFileService = data.resolve_ref();

    file_service.upload_file(upload_data.file).await.unwrap();
    "File uploaded"
}

#[tokio::main]
async fn main() -> Result<()> {
    logger_init().await?;

    let app_module = Arc::new(AppModule::builder()
        .build());
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
