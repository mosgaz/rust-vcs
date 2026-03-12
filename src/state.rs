use crate::errors::AppError;
use crate::models::{
    ChatMessage, ChatThread, DesktopAppStatus, DirectMessage, Meeting, RecordingSession,
    SignalEvent, ThreadMessage, User,
};
use argon2::password_hash::{rand_core::OsRng, SaltString};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct AppState {
    pub store: Arc<RwLock<Store>>,
    jwt_secret: Arc<String>,
}

#[derive(Debug)]
pub struct Store {
    pub users_by_id: HashMap<Uuid, User>,
    pub users_by_email: HashMap<String, Uuid>,
    pub meetings_by_slug: HashMap<String, Meeting>,
    pub room_channels: HashMap<String, broadcast::Sender<String>>,
    pub signal_channels: HashMap<String, broadcast::Sender<String>>,
    pub dms: Vec<DirectMessage>,
    pub room_messages: Vec<ChatMessage>,
    pub signal_events: Vec<SignalEvent>,
    pub threads_by_id: HashMap<Uuid, ChatThread>,
    pub thread_messages: Vec<ThreadMessage>,
    pub recordings: Vec<RecordingSession>,
    pub desktop_statuses: Vec<DesktopAppStatus>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Claims {
    sub: String,
    exp: usize,
}

impl AppState {
    pub fn bootstrap(jwt_secret: &str) -> Self {
        Self {
            store: Arc::new(RwLock::new(Store {
                users_by_id: HashMap::new(),
                users_by_email: HashMap::new(),
                meetings_by_slug: HashMap::new(),
                room_channels: HashMap::new(),
                signal_channels: HashMap::new(),
                dms: vec![],
                room_messages: vec![],
                signal_events: vec![],
                threads_by_id: HashMap::new(),
                thread_messages: vec![],
                recordings: vec![],
                desktop_statuses: vec![
                    DesktopAppStatus {
                        target: "windows".into(),
                        status: "planned".into(),
                        runtime: "tauri".into(),
                    },
                    DesktopAppStatus {
                        target: "macos".into(),
                        status: "planned".into(),
                        runtime: "tauri".into(),
                    },
                    DesktopAppStatus {
                        target: "linux".into(),
                        status: "planned".into(),
                        runtime: "tauri".into(),
                    },
                ],
            })),
            jwt_secret: Arc::new(jwt_secret.to_string()),
        }
    }

    pub fn hash_password(&self, password: &str) -> Result<String, AppError> {
        let salt = SaltString::generate(&mut OsRng);
        Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map(|h| h.to_string())
            .map_err(|_| AppError::Internal)
    }

    pub fn verify_password(&self, hash: &str, password: &str) -> bool {
        PasswordHash::new(hash)
            .ok()
            .and_then(|parsed| {
                Argon2::default()
                    .verify_password(password.as_bytes(), &parsed)
                    .ok()
            })
            .is_some()
    }

    pub fn issue_token(&self, user_id: Uuid) -> Result<String, AppError> {
        let claims = Claims {
            sub: user_id.to_string(),
            exp: (Utc::now().timestamp() + 24 * 3600) as usize,
        };
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
        .map_err(|_| AppError::Internal)
    }

    pub fn parse_token(&self, token: &str) -> Result<Uuid, AppError> {
        let token = token.trim().strip_prefix("Bearer ").unwrap_or(token);
        let decoded = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| AppError::InvalidCredentials)?;
        Uuid::parse_str(&decoded.claims.sub).map_err(|_| AppError::InvalidCredentials)
    }
}

impl Store {
    pub fn ensure_room_channel(&mut self, slug: &str) -> broadcast::Sender<String> {
        self.room_channels
            .entry(slug.to_string())
            .or_insert_with(|| {
                let (tx, _rx) = broadcast::channel(256);
                tx
            })
            .clone()
    }

    pub fn ensure_signal_channel(&mut self, slug: &str) -> broadcast::Sender<String> {
        self.signal_channels
            .entry(slug.to_string())
            .or_insert_with(|| {
                let (tx, _rx) = broadcast::channel(256);
                tx
            })
            .clone()
    }
}
