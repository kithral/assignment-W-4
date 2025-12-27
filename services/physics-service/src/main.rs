//! Physics Service - Physics simulation

use axum::{routing::get, Router};
use std::net::SocketAddr;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "physics-service=debug,info".into()),
        )
        .init();

    info!("🚀 Physics Service - Physics simulation starting...");

    dotenvy::dotenv().ok();

    let health_app = Router::new()
        .route("/health", get(health_check))
        .route("/ready", get(readiness_check));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8085));
    info!("Health check server listening on {}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await?, health_app).await?;

    Ok(())
}

async fn health_check() -> &'static str {
    "OK"
}

async fn readiness_check() -> &'static str {
    "READY"
}
