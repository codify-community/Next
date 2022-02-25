use anyhow::Result;
use std::env;
use tracing_subscriber::{EnvFilter, FmtSubscriber};
pub fn start_logger() -> Result<()> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .pretty()
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;
    Ok(())
}
