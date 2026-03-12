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
        .route("/", get(handlers::ui_index))
        .route("/health", get(handlers::health))
        .route("/v1/auth/register", post(handlers::register_employee))
        .route("/v1/auth/login", post(handlers::login))
        .route("/v1/meetings", post(handlers::create_meeting))
        .route("/v1/meetings/:slug/join", post(handlers::join_by_link))
        .route("/v1/meetings/:slug/ws", get(handlers::room_ws))
        .route("/v1/meetings/:slug/signal/ws", get(handlers::signal_ws))
        .route(
            "/v1/meetings/:slug/webinar/speakers",
            post(handlers::set_webinar_speaker),
        )
        .route(
            "/v1/meetings/:slug/recordings/start",
            post(handlers::start_recording),
        )
        .route("/v1/messages/direct", post(handlers::send_dm))
        .route("/v1/messenger/threads", post(handlers::create_thread))
        .route("/v1/messenger/threads", get(handlers::list_threads))
        .route(
            "/v1/messenger/threads/:thread_id/messages",
            post(handlers::send_thread_message),
        )
        .route(
            "/v1/messenger/threads/:thread_id/messages",
            get(handlers::list_thread_messages),
        )
        .route("/v1/desktop/status", get(handlers::desktop_status))
        .with_state(state)
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
}
