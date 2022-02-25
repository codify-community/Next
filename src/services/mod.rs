pub mod logger;
use anyhow::Result;
pub fn init() -> Result<()> {
    logger::start_logger()?;
    tracing::info!("[SERVICE] Logger started");
    Ok(())
}