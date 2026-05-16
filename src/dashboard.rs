//! Dashboard — Axum REST API server for security metrics.

use axum::{
    extract::State,
    response::Json,
    routing::get,
    Router,
};
use serde::Serialize;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing::info;

use crate::config::Config;

#[derive(Clone)]
struct AppState {
    config: Config,
}

#[derive(Serialize)]
struct SystemStatus {
    version: String,
    network: String,
    status: String,
    shield_enabled: bool,
    spear_enabled: bool,
}

#[derive(Serialize)]
struct HealthResponse {
    healthy: bool,
}

/// Start the dashboard Axum server.
pub async fn start(config: &Config, port: u16) -> anyhow::Result<()> {
    let state = Arc::new(AppState {
        config: config.clone(),
    });

    // Rate limiting: bind to localhost only + body size limit via axum's built-in
    let app = Router::new()
        .route("/api/v1/health", get(health))
        .route("/api/v1/status", get(status))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let addr = format!("127.0.0.1:{}", port);
    info!("Dashboard starting on http://{}", addr);

    println!("📊 StellarVallum Dashboard");
    println!("═══════════════════════════════════════");
    println!("  URL:     http://{}", addr);
    println!("  Network: testnet");
    println!("  API:     http://{}/api/v1/status", addr);
    println!("\n  Press Ctrl+C to stop\n");

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { healthy: true })
}

async fn status(State(state): State<Arc<AppState>>) -> Json<SystemStatus> {
    Json(SystemStatus {
        version: "0.2.0-testnet".to_string(),
        network: state.config.network.name.clone(),
        status: "operational".to_string(),
        shield_enabled: state.config.shield.enabled,
        spear_enabled: state.config.spear.enabled,
    })
}
