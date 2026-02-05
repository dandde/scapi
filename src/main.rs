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
    // Start server
    let listener = match tokio::net::TcpListener::bind(addr).await {
        Ok(l) => l,
        Err(e) => {
            if e.kind() == std::io::ErrorKind::AddrInUse {
                tracing::error!("Port {} is already in use.", config.server.port);
                tracing::error!(
                    "You can check what's running using: lsof -i :{}",
                    config.server.port
                );
                // tracing::error!("Or kill it: kill $(lsof -t -i :{})", config.server.port); // Optional tip
            }
            return Err(e.into());
        }
    };

    tracing::info!("Server started successfully");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

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

/// Graceful shutdown signal handler
async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("Shutdown signal received, starting graceful shutdown...");
}
