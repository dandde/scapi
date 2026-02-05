//! Tracing setup and configuration.

use tracing_subscriber::{EnvFilter, Layer, fmt, prelude::*};

/// Initialize logging for the application.
pub fn init_logging() -> Result<(), Box<dyn std::error::Error>> {
    // Create env filter from RUST_LOG environment variable
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info,scapi=debug"));

    // Choose format based on environment
    let format_layer = if std::env::var("RUST_LOG_FORMAT")
        .map(|v| v == "json")
        .unwrap_or(false)
    {
        fmt::layer().json().boxed()
    } else {
        fmt::layer().pretty().boxed()
    };

    // Initialize subscriber
    tracing_subscriber::registry()
        .with(env_filter)
        .with(format_layer)
        .init();

    tracing::info!("Logging initialized");

    Ok(())
}
