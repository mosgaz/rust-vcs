use crate::errors::AppError;
use crate::i18n;
use crate::models::{
    ApiMessage, ChatMessage, CreateMeetingRequest, CreateMeetingResponse, DirectMessage,
    FileAttachment, JoinByLinkRequest, JoinByLinkResponse, LoginRequest, LoginResponse,
    RegisterEmployeeRequest, RegisterEmployeeResponse, SendDirectMessageRequest, SignalEvent,
};
use crate::state::AppState;
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::Html;
use axum::response::IntoResponse;
use axum::Json;
use chrono::Utc;
use futures::{sink::SinkExt, stream::StreamExt};
use serde::Deserialize;
use uuid::Uuid;

pub async fn ui_index() -> Html<String> {
    Html(crate::ui::render_landing_page())
}

pub async fn health(headers: HeaderMap) -> Json<ApiMessage> {
    let lang = i18n::locale(&headers);
    Json(ApiMessage {
        message: i18n::t("health", lang).to_string(),
    })
}

pub async fn register_employee(
    State(state): State<AppState>,
    Json(req): Json<RegisterEmployeeRequest>,
) -> Result<(StatusCode, Json<RegisterEmployeeResponse>), AppError> {
    if req.email.trim().is_empty() || req.password.len() < 8 {
        return Err(AppError::BadRequest(
            "email must be provided and password must be at least 8 characters".into(),
        ));
    }

    let mut store = state.store.write().await;
    if store.users_by_email.contains_key(&req.email) {
        return Err(AppError::BadRequest("email already exists".into()));
    }
    let user_id = Uuid::new_v4();
    let password_hash = state.hash_password(&req.password)?;
    let user = crate::models::User {
        id: user_id,
        email: req.email.clone(),
        display_name: req.display_name,
        password_hash,
    };
    store.users_by_email.insert(user.email.clone(), user_id);
    store.users_by_id.insert(user_id, user);

    Ok((
        StatusCode::CREATED,
        Json(RegisterEmployeeResponse { user_id }),
    ))
}

pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    let store = state.store.read().await;
    let user_id = store
        .users_by_email
        .get(&req.email)
        .ok_or(AppError::InvalidCredentials)?;
    let user = store
        .users_by_id
        .get(user_id)
        .ok_or(AppError::InvalidCredentials)?;
    if !state.verify_password(&user.password_hash, &req.password) {
        return Err(AppError::InvalidCredentials);
    }
    drop(store);

    Ok(Json(LoginResponse {
        access_token: state.issue_token(*user_id)?,
    }))
}

pub async fn create_meeting(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<CreateMeetingRequest>,
) -> Result<Json<CreateMeetingResponse>, AppError> {
    let auth = headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or(AppError::InvalidCredentials)?;
    let organizer_id = state.parse_token(auth)?;

    let id = Uuid::new_v4();
    let slug = format!("m-{}", &id.to_string()[..8]);
    let meeting = crate::models::Meeting {
        id,
        slug: slug.clone(),
        title: req.title,
        organizer_id,
        created_at: Utc::now(),
    };

    let mut store = state.store.write().await;
    store.meetings_by_slug.insert(slug.clone(), meeting);
    store.ensure_room_channel(&slug);
    store.ensure_signal_channel(&slug);

    Ok(Json(CreateMeetingResponse {
        meeting_id: id,
        invite_link: format!("/v1/meetings/{slug}/join"),
    }))
}

pub async fn join_by_link(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Json(req): Json<JoinByLinkRequest>,
) -> Result<Json<JoinByLinkResponse>, AppError> {
    let mut store = state.store.write().await;
    if !store.meetings_by_slug.contains_key(&slug) {
        return Err(AppError::MeetingNotFound);
    }
    store.ensure_room_channel(&slug);
    store.ensure_signal_channel(&slug);

    let joined = ChatMessage {
        id: Uuid::new_v4(),
        room_slug: slug.clone(),
        sender_id: None,
        sender_name: req.guest_name,
        content: "joined the room".into(),
        file: None,
        sent_at: Utc::now(),
    };
    store.room_messages.push(joined);

    Ok(Json(JoinByLinkResponse {
        room_slug: slug.clone(),
        ws_url: format!("/v1/meetings/{slug}/ws"),
        signal_ws_url: format!("/v1/meetings/{slug}/signal/ws"),
    }))
}

pub async fn signal_ws(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    {
        let store = state.store.read().await;
        if !store.meetings_by_slug.contains_key(&slug) {
            return Err(AppError::MeetingNotFound);
        }
    }

    Ok(ws.on_upgrade(move |socket| async move {
        handle_signal_socket(state, slug, socket).await;
    }))
}

async fn handle_signal_socket(state: AppState, slug: String, socket: WebSocket) {
    let signal_tx = {
        let mut store = state.store.write().await;
        store.ensure_signal_channel(&slug)
    };
    let mut signal_rx = signal_tx.subscribe();

    let (mut sender, mut receiver) = socket.split();

    let send_task = tokio::spawn(async move {
        while let Ok(message) = signal_rx.recv().await {
            if sender.send(Message::Text(message.into())).await.is_err() {
                break;
            }
        }
    });

    let receive_state = state.clone();
    let receive_slug = slug.clone();
    let receive_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if let Message::Text(text) = msg {
                let event = SignalEvent {
                    id: Uuid::new_v4(),
                    room_slug: receive_slug.clone(),
                    payload: text.to_string(),
                    sent_at: Utc::now(),
                };
                let mut store = receive_state.store.write().await;
                store.signal_events.push(event);
                if let Some(channel) = store.signal_channels.get(&receive_slug) {
                    let _ = channel.send(text.to_string());
                }
            }
        }
    });

    let _ = tokio::join!(send_task, receive_task);
}

#[derive(Debug, Deserialize)]
struct IncomingRoomMessage {
    sender_name: String,
    content: String,
    file: Option<FileAttachment>,
}

pub async fn room_ws(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    {
        let store = state.store.read().await;
        if !store.meetings_by_slug.contains_key(&slug) {
            return Err(AppError::MeetingNotFound);
        }
    }

    Ok(ws.on_upgrade(move |socket| async move {
        handle_room_socket(state, slug, socket).await;
    }))
}

async fn handle_room_socket(state: AppState, slug: String, socket: WebSocket) {
    let room_tx = {
        let mut store = state.store.write().await;
        store.ensure_room_channel(&slug)
    };
    let mut room_rx = room_tx.subscribe();

    let (mut sender, mut receiver) = socket.split();

    let send_task = tokio::spawn(async move {
        while let Ok(message) = room_rx.recv().await {
            if sender.send(Message::Text(message.into())).await.is_err() {
                break;
            }
        }
    });

    let receive_state = state.clone();
    let receive_slug = slug.clone();
    let receive_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if let Message::Text(text) = msg {
                let Ok(input) = serde_json::from_str::<IncomingRoomMessage>(&text) else {
                    continue;
                };
                let event = ChatMessage {
                    id: Uuid::new_v4(),
                    room_slug: receive_slug.clone(),
                    sender_id: None,
                    sender_name: input.sender_name,
                    content: input.content,
                    file: input.file,
                    sent_at: Utc::now(),
                };
                let payload = serde_json::to_string(&event).unwrap_or_default();
                let mut store = receive_state.store.write().await;
                store.room_messages.push(event);
                if let Some(channel) = store.room_channels.get(&receive_slug) {
                    let _ = channel.send(payload);
                }
            }
        }
    });

    let _ = tokio::join!(send_task, receive_task);
}

pub async fn send_dm(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<SendDirectMessageRequest>,
) -> Result<(StatusCode, Json<ApiMessage>), AppError> {
    let auth = headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or(AppError::InvalidCredentials)?;
    let from_user_id = state.parse_token(auth)?;

    let mut store = state.store.write().await;
    if !store.users_by_id.contains_key(&req.to_user_id) {
        return Err(AppError::UserNotFound);
    }
    store.dms.push(DirectMessage {
        id: Uuid::new_v4(),
        from_user_id,
        to_user_id: req.to_user_id,
        text: req.text,
        sent_at: Utc::now(),
    });

    let lang = i18n::locale(&headers);
    Ok((
        StatusCode::CREATED,
        Json(ApiMessage {
            message: i18n::t("dm_sent", lang).to_string(),
        }),
    ))
}
