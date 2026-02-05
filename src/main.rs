//! SCAPI Server entry point.

use std::net::SocketAddr;

use scapi::AppState;
use scapi::api;
use scapi::infra::config::AppConfig;
use scapi::infra::logging::tracer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracer::init_logging()?;

    // Load configuration
    let config = AppConfig::from_env()?;

    // Print banner
    print_banner();

    tracing::info!("Starting SCAPI server...");
    tracing::debug!("Configuration loaded successfully");

    // Initialize application state
    let state = AppState::new()?;

    // Create router
    let app = api::create_router(state);

    // Determine bind address
    let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));
    tracing::info!("Listening on {}", addr);

    // Start server
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

fn print_banner() {
    println!(
        r#"
███████╗ ██████╗ █████╗ ██████╗ ██╗
██╔════╝██╔════╝██╔══██╗██╔══██╗██║
███████╗██║     ███████║██████╔╝██║
╚════██║██║     ██╔══██║██╔═══╝ ██║
███████║╚██████╗██║  ██║██║     ██║
╚══════╝ ╚═════╝╚═╝  ╚═╝╚═╝     ╚═╝
                                        
    Scalable CSS/XPath API v0.1.0
    "#
    );
}
