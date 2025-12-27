//! World State Service - Core ECS and Authoritative Game State
//!
//! This service is the heart of the game engine, maintaining the authoritative
//! state of the world using Bevy ECS. It coordinates with other services via
//! gRPC and distributes events to subscribers.

use axum::{routing::get, Router};
use std::net::SocketAddr;
use tracing::{info, warn};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "world_state=debug,info".into()),
        )
        .init();

    info!("🌍 World State Service starting...");

    // Load environment variables
    dotenvy::dotenv().ok();

    // Health check server
    let health_app = Router::new()
        .route("/health", get(health_check))
        .route("/ready", get(readiness_check));

    let health_addr = SocketAddr::from(([0, 0, 0, 0], 8080));
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

    info!("✅ World State Service ready");

    // Keep the service running
    tokio::signal::ctrl_c().await?;
    info!("Shutting down World State Service");

    Ok(())
}

/// Health check endpoint for Kubernetes liveness probe
async fn health_check() -> &'static str {
    "OK"
}

/// Readiness check endpoint for Kubernetes readiness probe
async fn readiness_check() -> &'static str {
    // TODO: Check dependencies (Redis, PostgreSQL, etc.)
    "READY"
}
