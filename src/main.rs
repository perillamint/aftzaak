use std::net::SocketAddr;
use std::sync::Arc;

use axum::Router;
use clap::Parser;
use sea_orm::Database;

use config::Config;
use util::tokensigner::TokenSigner;

pub mod api;
pub mod config;
pub mod entity;
pub mod error;
pub mod types;
pub mod util;

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value = "config.toml")]
    config: String,
}

#[derive(Clone)]
pub struct AppState {
    pub db: sea_orm::DatabaseConnection,
    pub tokensigner: Arc<TokenSigner>,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let args = Args::parse();
    let cfg = Config::parse_config_file(&args.config).await?;

    let db = Database::connect(&cfg.db.database_url).await?;

    let tokensigner = TokenSigner::new(cfg.jwt.clone());

    let state = Arc::new(AppState {
        db,
        tokensigner: Arc::new(tokensigner),
    });

    let app = Router::new()
        .nest("/api", api::get_router())
        .with_state(state);

    let addr = SocketAddr::from((cfg.http.listen, cfg.http.port));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!("listening on {addr}");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

// Code stolen from axum example. MIT.
async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("install ctrl_c handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("install terminate handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {}
        _ = terminate => {}
    }

    tracing::info!("shutdown signal received");
}
