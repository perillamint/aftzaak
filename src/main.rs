use std::net::SocketAddr;
use std::sync::Arc;

use axum::Router;
use axum::routing::get;
use clap::Parser;
use sea_orm::Database;

use config::Config;

mod config;
mod api;

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value = "config.toml")]
    config: String,
}

struct AppState {
    #[allow(dead_code)]
    db: sea_orm::DatabaseConnection,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt::init();

    let args = Args::parse();
    let cfg = Config::parse_config_file(&args.config).await?;

    let db = Database::connect(&cfg.db.database_url).await?;

    let state = Arc::new(AppState { db });

    let app = Router::new().route("/", get(hello)).with_state(state);

    let addr = SocketAddr::from((cfg.http.listen, cfg.http.port));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!("listening on {addr}");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn hello() -> String {
    String::from("Hello, world!")
}

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
