use crate::errors::AppError;
use crate::i18n;
use crate::models::{
    ApiMessage, ChatMessage, ChatThread, CreateMeetingRequest, CreateMeetingResponse,
    CreateThreadRequest, CreateThreadResponse, DirectMessage, FileAttachment, JoinByLinkRequest,
    JoinByLinkResponse, LoginRequest, LoginResponse, MeetingMode, MeetingSpeakersResponse,
    RecordingSession, RegisterEmployeeRequest, RegisterEmployeeResponse, SendDirectMessageRequest,
    SendThreadMessageRequest, SetSpeakerRequest, SignalEvent, StartRecordingResponse,
    SystemStatusResponse, ThreadMessage,
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

fn bearer_user(headers: &HeaderMap, state: &AppState) -> Result<Uuid, AppError> {
    let auth = headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or(AppError::InvalidCredentials)?;
    state.parse_token(auth)
}

pub async fn ui_index() -> Html<String> {
    Html(crate::ui::render_landing_page())
}

pub async fn ui_login() -> Html<String> {
    Html(crate::ui::render_login_page())
}

pub async fn ui_register() -> Html<String> {
    Html(crate::ui::render_register_page())
}

pub async fn ui_messenger() -> Html<String> {
    Html(crate::ui::render_messenger_page())
}

pub async fn ui_meeting(Path(slug): Path<String>) -> Html<String> {
    Html(crate::ui::render_meeting_page(&slug))
}

pub async fn ui_waiting_room(Path(slug): Path<String>) -> Html<String> {
    Html(crate::ui::render_waiting_room_page(&slug))
}

pub async fn system_status() -> Json<SystemStatusResponse> {
    Json(SystemStatusResponse {
        stage: "stage2".into(),
        auth: true,
        meetings: true,
        chat_ws: true,
        signaling_ws: true,
        messenger_threads: true,
        webinar_mode: true,
        recording_placeholder: true,
        mediasoup_sfu: false,
    })
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
    let user_id = *store
        .users_by_email
        .get(&req.email)
        .ok_or(AppError::InvalidCredentials)?;
    let user = store
        .users_by_id
        .get(&user_id)
        .ok_or(AppError::InvalidCredentials)?;
    if !state.verify_password(&user.password_hash, &req.password) {
        return Err(AppError::InvalidCredentials);
    }
    drop(store);

    Ok(Json(LoginResponse {
        access_token: state.issue_token(user_id)?,
    }))
}

pub async fn create_meeting(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<CreateMeetingRequest>,
) -> Result<Json<CreateMeetingResponse>, AppError> {
    let organizer_id = bearer_user(&headers, &state)?;

    let id = Uuid::new_v4();
    let slug = format!("m-{}", &id.to_string()[..8]);
    let mode = req.mode.unwrap_or(MeetingMode::Meeting);
    let meeting = crate::models::Meeting {
        id,
        slug: slug.clone(),
        title: req.title,
        organizer_id,
        mode: mode.clone(),
        speaker_ids: vec![organizer_id],
        created_at: Utc::now(),
    };

    let mut store = state.store.write().await;
    store.meetings_by_slug.insert(slug.clone(), meeting);
    store.ensure_room_channel(&slug);
    store.ensure_signal_channel(&slug);

    Ok(Json(CreateMeetingResponse {
        meeting_id: id,
        invite_link: format!("/v1/meetings/{slug}/join"),
        mode,
    }))
}

pub async fn set_webinar_speaker(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(slug): Path<String>,
    Json(req): Json<SetSpeakerRequest>,
) -> Result<Json<MeetingSpeakersResponse>, AppError> {
    let requester = bearer_user(&headers, &state)?;
    let mut store = state.store.write().await;
    if !store.users_by_id.contains_key(&req.user_id) {
        return Err(AppError::UserNotFound);
    }

    let meeting = store
        .meetings_by_slug
        .get_mut(&slug)
        .ok_or(AppError::MeetingNotFound)?;

    if meeting.organizer_id != requester {
        return Err(AppError::InvalidCredentials);
    }
    if meeting.mode != MeetingMode::Webinar {
        return Err(AppError::BadRequest(
            "speaker management is available only for webinar mode".into(),
        ));
    }
    if !meeting.speaker_ids.contains(&req.user_id) {
        meeting.speaker_ids.push(req.user_id);
    }

    Ok(Json(MeetingSpeakersResponse {
        room_slug: slug,
        mode: meeting.mode.clone(),
        speaker_ids: meeting.speaker_ids.clone(),
    }))
}

pub async fn start_recording(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(slug): Path<String>,
) -> Result<(StatusCode, Json<StartRecordingResponse>), AppError> {
    let started_by = bearer_user(&headers, &state)?;
    let mut store = state.store.write().await;

    let meeting = store
        .meetings_by_slug
        .get(&slug)
        .ok_or(AppError::MeetingNotFound)?;
    if meeting.organizer_id != started_by {
        return Err(AppError::InvalidCredentials);
    }

    let recording = RecordingSession {
        id: Uuid::new_v4(),
        room_slug: slug.clone(),
        started_by,
        status: "recording".into(),
        storage_uri: format!("s3://rust-vcs-recordings/{slug}/{}.mp4", Uuid::new_v4()),
        started_at: Utc::now(),
    };

    store.recordings.push(recording.clone());

    Ok((
        StatusCode::CREATED,
        Json(StartRecordingResponse {
            recording_id: recording.id,
            room_slug: recording.room_slug,
            status: recording.status,
            storage_uri: recording.storage_uri,
        }),
    ))
}

pub async fn desktop_status(
    State(state): State<AppState>,
) -> Json<Vec<crate::models::DesktopAppStatus>> {
    let store = state.store.read().await;
    Json(store.desktop_statuses.clone())
}

pub async fn create_thread(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<CreateThreadRequest>,
) -> Result<(StatusCode, Json<CreateThreadResponse>), AppError> {
    let created_by = bearer_user(&headers, &state)?;
    if req.title.trim().is_empty() {
        return Err(AppError::BadRequest("thread title is required".into()));
    }

    let mut participants = req.participant_ids;
    if !participants.contains(&created_by) {
        participants.push(created_by);
    }

    let mut store = state.store.write().await;
    if participants
        .iter()
        .any(|id| !store.users_by_id.contains_key(id))
    {
        return Err(AppError::UserNotFound);
    }

    let thread_id = Uuid::new_v4();
    store.threads_by_id.insert(
        thread_id,
        ChatThread {
            id: thread_id,
            title: req.title,
            is_channel: req.is_channel,
            participant_ids: participants,
            created_by,
            created_at: Utc::now(),
        },
    );

    Ok((
        StatusCode::CREATED,
        Json(CreateThreadResponse { thread_id }),
    ))
}

pub async fn list_threads(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Vec<ChatThread>>, AppError> {
    let user_id = bearer_user(&headers, &state)?;
    let store = state.store.read().await;
    let threads = store
        .threads_by_id
        .values()
        .filter(|thread| thread.participant_ids.contains(&user_id) || thread.is_channel)
        .cloned()
        .collect();
    Ok(Json(threads))
}

pub async fn send_thread_message(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(thread_id): Path<Uuid>,
    Json(req): Json<SendThreadMessageRequest>,
) -> Result<(StatusCode, Json<ApiMessage>), AppError> {
    let sender_id = bearer_user(&headers, &state)?;
    let mut store = state.store.write().await;
    let thread = store
        .threads_by_id
        .get(&thread_id)
        .ok_or(AppError::BadRequest("thread not found".into()))?;

    if !thread.is_channel && !thread.participant_ids.contains(&sender_id) {
        return Err(AppError::InvalidCredentials);
    }

    store.thread_messages.push(ThreadMessage {
        id: Uuid::new_v4(),
        thread_id,
        sender_id,
        text: req.text,
        sent_at: Utc::now(),
    });

    Ok((
        StatusCode::CREATED,
        Json(ApiMessage {
            message: "thread message sent".into(),
        }),
    ))
}

pub async fn list_thread_messages(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(thread_id): Path<Uuid>,
) -> Result<Json<Vec<ThreadMessage>>, AppError> {
    let user_id = bearer_user(&headers, &state)?;
    let store = state.store.read().await;
    let thread = store
        .threads_by_id
        .get(&thread_id)
        .ok_or(AppError::BadRequest("thread not found".into()))?;
    if !thread.is_channel && !thread.participant_ids.contains(&user_id) {
        return Err(AppError::InvalidCredentials);
    }

    let messages = store
        .thread_messages
        .iter()
        .filter(|message| message.thread_id == thread_id)
        .cloned()
        .collect();
    Ok(Json(messages))
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
    let from_user_id = bearer_user(&headers, &state)?;

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
