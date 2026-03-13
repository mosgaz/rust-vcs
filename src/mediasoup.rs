use crate::errors::AppError;
use reqwest::Client;
use serde::Serialize;
use std::env;

#[derive(Debug, Clone)]
pub struct MediaSoupClient {
    enabled: bool,
    api_url: String,
    http: Client,
}

#[derive(Debug, Serialize)]
struct EnsureRoomRequest<'a> {
    room_slug: &'a str,
}

impl MediaSoupClient {
    pub fn from_env() -> Self {
        let enabled = env::var("MEDIASOUP_ENABLED")
            .ok()
            .map(|v| matches!(v.as_str(), "1" | "true" | "TRUE" | "yes" | "on"))
            .unwrap_or(false);
        let api_url = env::var("MEDIASOUP_API_URL")
            .unwrap_or_else(|_| "http://127.0.0.1:4000".to_string())
            .trim_end_matches('/')
            .to_string();

        Self {
            enabled,
            api_url,
            http: Client::new(),
        }
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn api_url(&self) -> &str {
        &self.api_url
    }

    pub async fn health(&self) -> bool {
        if !self.enabled {
            return false;
        }

        self.http
            .get(format!("{}/health", self.api_url))
            .send()
            .await
            .map(|r| r.status().is_success())
            .unwrap_or(false)
    }

    pub async fn ensure_room(&self, slug: &str) -> Result<(), AppError> {
        if !self.enabled {
            return Ok(());
        }

        let response = self
            .http
            .post(format!("{}/rooms/ensure", self.api_url))
            .json(&EnsureRoomRequest { room_slug: slug })
            .send()
            .await
            .map_err(|e| {
                AppError::MediaSoupUnavailable(format!("mediasoup request failed: {e}"))
            })?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(AppError::MediaSoupUnavailable(format!(
                "mediasoup returned {}",
                response.status()
            )))
        }
    }
}
