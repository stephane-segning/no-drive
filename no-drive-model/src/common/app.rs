use anyhow::Result;
use log;

pub async fn logger_init() -> Result<()> {
    pretty_env_logger::init();

    // tracing_log::LogTracer::builder().init()?;

    Ok(())
}