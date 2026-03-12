use axum::http::HeaderMap;

pub fn locale(headers: &HeaderMap) -> &'static str {
    let header = headers
        .get(axum::http::header::ACCEPT_LANGUAGE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("en");

    if header.starts_with("ru") {
        "ru"
    } else {
        "en"
    }
}

pub fn t(key: &str, lang: &str) -> &'static str {
    match (key, lang) {
        ("health", "ru") => "Сервис rust-vcs MVP работает",
        ("health", _) => "rust-vcs MVP is running",
        ("dm_sent", "ru") => "Личное сообщение отправлено",
        ("dm_sent", _) => "Direct message sent",
        _ => "ok",
    }
}
