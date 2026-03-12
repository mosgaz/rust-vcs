use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use rust_vcs::app::build_router;
use rust_vcs::state::AppState;
use serde_json::{json, Value};
use tower::ServiceExt;

#[tokio::test]
async fn health_works_with_i18n() {
    let app = build_router(AppState::bootstrap("secret"));
    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .header("Accept-Language", "ru")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let payload: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(payload["message"], "Сервис rust-vcs MVP работает");
}

#[tokio::test]
async fn leptos_tailwind_ui_page_is_served() {
    let app = build_router(AppState::bootstrap("secret"));
    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let html = String::from_utf8(body.to_vec()).unwrap();
    assert!(html.contains("Leptos + Tailwind"));
    assert!(html.contains("cdn.tailwindcss.com"));
}

#[tokio::test]
async fn employee_can_register_login_and_create_meeting() {
    let app = build_router(AppState::bootstrap("secret"));

    let register_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/auth/register")
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({
                        "email": "alice@corp.local",
                        "password": "super-secret-1",
                        "display_name": "Alice"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(register_response.status(), StatusCode::CREATED);

    let login_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/auth/login")
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({"email": "alice@corp.local", "password": "super-secret-1"}).to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(login_response.status(), StatusCode::OK);
    let body = login_response
        .into_body()
        .collect()
        .await
        .unwrap()
        .to_bytes();
    let token: Value = serde_json::from_slice(&body).unwrap();

    let create_meeting_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/meetings")
                .header("content-type", "application/json")
                .header(
                    "authorization",
                    format!("Bearer {}", token["access_token"].as_str().unwrap()),
                )
                .body(Body::from(json!({"title": "Stage 1 Demo"}).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(create_meeting_response.status(), StatusCode::OK);
}
