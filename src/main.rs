use rust_vcs::app::build_router;
use rust_vcs::state::AppState;
use rust_vcs::tls;
use std::env;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let _tls_config = tls::build_placeholder_tls_config();
    let state = AppState::bootstrap("change-me-in-prod");
    let app = build_router(state);

    let port = env::var("RUST_VCS_PORT")
        .or_else(|_| env::var("PORT"))
        .ok()
        .and_then(|raw| raw.parse::<u16>().ok())
        .unwrap_or(8080);
    let addr: SocketAddr = format!("0.0.0.0:{port}").parse()?;
    let listener = TcpListener::bind(addr).await?;
    info!("rust-vcs mvp listening on {}", addr);

    axum::serve(listener, app).await?;
    Ok(())
}
