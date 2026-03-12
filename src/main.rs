use rust_vcs::app::build_router;
use rust_vcs::state::AppState;
use rust_vcs::tls;
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

    let addr: SocketAddr = "0.0.0.0:8080".parse()?;
    let listener = TcpListener::bind(addr).await?;
    info!("rust-vcs mvp listening on {}", addr);

    axum::serve(listener, app).await?;
    Ok(())
}
