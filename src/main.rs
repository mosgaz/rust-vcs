use rust_vcs::app::build_router;
use rust_vcs::state::AppState;
use rust_vcs::tls;
use std::env;
use std::fs;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::info;

fn load_env_from_dotenv() {
    let Ok(raw) = fs::read_to_string(".env") else {
        return;
    };

    for line in raw.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        let key = key.trim();
        if key.is_empty() || env::var_os(key).is_some() {
            continue;
        }

        let mut value = value.trim().to_string();
        if (value.starts_with('"') && value.ends_with('"'))
            || (value.starts_with('\'') && value.ends_with('\''))
        {
            value = value[1..value.len() - 1].to_string();
        }

        env::set_var(key, value);
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let _tls_config = tls::build_placeholder_tls_config();
    load_env_from_dotenv();
    let state = AppState::bootstrap("change-me-in-prod");
    let app = build_router(state);

    let port = env::var("RUST_VCS_PORT")
        .or_else(|_| env::var("PORT"))
        .ok()
        .and_then(|raw| raw.parse::<u16>().ok())
        .unwrap_or(3242);
    let addr: SocketAddr = format!("0.0.0.0:{port}").parse()?;
    let listener = TcpListener::bind(addr).await?;
    info!("rust-vcs mvp listening on {}", addr);

    axum::serve(listener, app).await?;
    Ok(())
}
