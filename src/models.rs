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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meeting {
    pub id: Uuid,
    pub slug: String,
    pub title: String,
    pub organizer_id: Uuid,
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
}

#[derive(Debug, Serialize)]
pub struct CreateMeetingResponse {
    pub meeting_id: Uuid,
    pub invite_link: String,
}

#[derive(Debug, Deserialize)]
pub struct JoinByLinkRequest {
    pub guest_name: String,
}

#[derive(Debug, Serialize)]
pub struct JoinByLinkResponse {
    pub room_slug: String,
    pub ws_url: String,
}

#[derive(Debug, Deserialize)]
pub struct SendDirectMessageRequest {
    pub to_user_id: Uuid,
    pub text: String,
}

#[derive(Debug, Serialize)]
pub struct ApiMessage {
    pub message: String,
}
