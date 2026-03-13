use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub display_name: String,
    pub password_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MeetingMode {
    Meeting,
    Webinar,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meeting {
    pub id: Uuid,
    pub slug: String,
    pub title: String,
    pub organizer_id: Uuid,
    pub mode: MeetingMode,
    pub speaker_ids: Vec<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: Uuid,
    pub room_slug: String,
    pub sender_id: Option<Uuid>,
    pub sender_name: String,
    pub content: String,
    pub file: Option<FileAttachment>,
    pub sent_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileAttachment {
    pub filename: String,
    pub mime_type: String,
    pub bytes_base64: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectMessage {
    pub id: Uuid,
    pub from_user_id: Uuid,
    pub to_user_id: Uuid,
    pub text: String,
    pub sent_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalEvent {
    pub id: Uuid,
    pub room_slug: String,
    pub payload: String,
    pub sent_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatThread {
    pub id: Uuid,
    pub title: String,
    pub is_channel: bool,
    pub participant_ids: Vec<Uuid>,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadMessage {
    pub id: Uuid,
    pub thread_id: Uuid,
    pub sender_id: Uuid,
    pub text: String,
    pub sent_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordingSession {
    pub id: Uuid,
    pub room_slug: String,
    pub started_by: Uuid,
    pub status: String,
    pub storage_uri: String,
    pub started_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesktopAppStatus {
    pub target: String,
    pub status: String,
    pub runtime: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterEmployeeRequest {
    pub email: String,
    pub password: String,
    pub display_name: String,
}

#[derive(Debug, Serialize)]
pub struct RegisterEmployeeResponse {
    pub user_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub access_token: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateMeetingRequest {
    pub title: String,
    #[serde(default)]
    pub mode: Option<MeetingMode>,
}

#[derive(Debug, Serialize)]
pub struct CreateMeetingResponse {
    pub meeting_id: Uuid,
    pub invite_link: String,
    pub mode: MeetingMode,
}

#[derive(Debug, Deserialize)]
pub struct JoinByLinkRequest {
    pub guest_name: String,
}

#[derive(Debug, Serialize)]
pub struct JoinByLinkResponse {
    pub room_slug: String,
    pub ws_url: String,
    pub signal_ws_url: String,
}

#[derive(Debug, Deserialize)]
pub struct SendDirectMessageRequest {
    pub to_user_id: Uuid,
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateThreadRequest {
    pub title: String,
    #[serde(default)]
    pub is_channel: bool,
    #[serde(default)]
    pub participant_ids: Vec<Uuid>,
}

#[derive(Debug, Serialize)]
pub struct CreateThreadResponse {
    pub thread_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct SendThreadMessageRequest {
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct SetSpeakerRequest {
    pub user_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct MeetingSpeakersResponse {
    pub room_slug: String,
    pub mode: MeetingMode,
    pub speaker_ids: Vec<Uuid>,
}

#[derive(Debug, Serialize)]
pub struct StartRecordingResponse {
    pub recording_id: Uuid,
    pub room_slug: String,
    pub status: String,
    pub storage_uri: String,
}

#[derive(Debug, Serialize)]
pub struct ApiMessage {
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct SystemStatusResponse {
    pub stage: String,
    pub mediasoup_enabled: bool,
    pub mediasoup_api_url: String,
    pub auth: bool,
    pub meetings: bool,
    pub chat_ws: bool,
    pub signaling_ws: bool,
    pub messenger_threads: bool,
    pub webinar_mode: bool,
    pub recording_placeholder: bool,
    pub mediasoup_sfu: bool,
}
