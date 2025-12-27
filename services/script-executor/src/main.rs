//! Script Executor Service - Sandboxed Script Execution
//!
//! This service executes user scripts in a safe, sandboxed environment.
//! It supports both Rhai (default) and Lua (placeholder for future).

use axum::{routing::get, Router};
use std::net::SocketAddr;
use tracing::{info, warn};

#[cfg(feature = "lua-scripting")]
mod lua_executor;
mod rhai_executor;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "script_executor=debug,info".into()),
        )
        .init();

    info!("📜 Script Executor Service starting...");

    // Load environment variables
    dotenvy::dotenv().ok();

    // Determine which scripting engine to use
    #[cfg(feature = "rhai-scripting")]
    info!("Using Rhai scripting engine");

    #[cfg(feature = "lua-scripting")]
    info!("Using Lua scripting engine");

    // Health check server
    let health_app = Router::new()
        .route("/health", get(health_check))
        .route("/ready", get(readiness_check));

    let health_addr = SocketAddr::from(([0, 0, 0, 0], 8081));
    info!("Health check server listening on {}", health_addr);

    tokio::spawn(async move {
        if let Err(e) = axum::serve(
            tokio::net::TcpListener::bind(health_addr).await.unwrap(),
            health_app,
        )
        .await
        {
            warn!("Health check server error: {}", e);
        }
    });

    info!("✅ Script Executor Service ready");

    // Keep the service running
    tokio::signal::ctrl_c().await?;
    info!("Shutting down Script Executor Service");

    Ok(())
}

/// Health check endpoint
async fn health_check() -> &'static str {
    "OK"
}

/// Readiness check endpoint
async fn readiness_check() -> &'static str {
    "READY"
}
