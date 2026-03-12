use crate::handlers;
use crate::state::AppState;
use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(handlers::health))
        .route("/v1/auth/register", post(handlers::register_employee))
        .route("/v1/auth/login", post(handlers::login))
        .route("/v1/meetings", post(handlers::create_meeting))
        .route("/v1/meetings/:slug/join", post(handlers::join_by_link))
        .route("/v1/meetings/:slug/ws", get(handlers::room_ws))
        .route("/v1/messages/direct", post(handlers::send_dm))
        .with_state(state)
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
}
