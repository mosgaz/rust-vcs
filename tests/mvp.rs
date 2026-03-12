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
    assert!(html.contains("Этап 2"));
    assert!(html.contains("cdn.tailwindcss.com"));
    assert!(html.contains("v1/messenger/threads"));
}

#[tokio::test]
async fn stage2_messenger_webinar_and_recording_flow() {
    let app = build_router(AppState::bootstrap("secret"));

    let register_alice = app
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
    assert_eq!(register_alice.status(), StatusCode::CREATED);
    let alice_body = register_alice
        .into_body()
        .collect()
        .await
        .unwrap()
        .to_bytes();
    let alice_payload: Value = serde_json::from_slice(&alice_body).unwrap();

    let register_bob = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/auth/register")
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({
                        "email": "bob@corp.local",
                        "password": "super-secret-2",
                        "display_name": "Bob"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(register_bob.status(), StatusCode::CREATED);

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
    let bearer = format!("Bearer {}", token["access_token"].as_str().unwrap());

    let create_thread = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/messenger/threads")
                .header("content-type", "application/json")
                .header("authorization", &bearer)
                .body(Body::from(
                    json!({
                        "title": "engineering",
                        "is_channel": false,
                        "participant_ids": [alice_payload["user_id"]]
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(create_thread.status(), StatusCode::CREATED);
    let thread_body = create_thread
        .into_body()
        .collect()
        .await
        .unwrap()
        .to_bytes();
    let thread_payload: Value = serde_json::from_slice(&thread_body).unwrap();

    let send_message = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/v1/messenger/threads/{}/messages",
                    thread_payload["thread_id"].as_str().unwrap()
                ))
                .header("content-type", "application/json")
                .header("authorization", &bearer)
                .body(Body::from(json!({"text": "hello stage 2"}).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(send_message.status(), StatusCode::CREATED);

    let create_webinar = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/meetings")
                .header("content-type", "application/json")
                .header("authorization", &bearer)
                .body(Body::from(
                    json!({"title": "Stage 2 Webinar", "mode": "webinar"}).to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(create_webinar.status(), StatusCode::OK);
    let webinar_body = create_webinar
        .into_body()
        .collect()
        .await
        .unwrap()
        .to_bytes();
    let webinar_payload: Value = serde_json::from_slice(&webinar_body).unwrap();
    assert_eq!(webinar_payload["mode"], "webinar");

    let slug = webinar_payload["invite_link"]
        .as_str()
        .unwrap()
        .trim_start_matches("/v1/meetings/")
        .trim_end_matches("/join")
        .to_string();

    let set_speaker = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/v1/meetings/{slug}/webinar/speakers"))
                .header("content-type", "application/json")
                .header("authorization", &bearer)
                .body(Body::from(
                    json!({"user_id": alice_payload["user_id"]}).to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(set_speaker.status(), StatusCode::OK);

    let recording_start = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/v1/meetings/{slug}/recordings/start"))
                .header("authorization", &bearer)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(recording_start.status(), StatusCode::CREATED);

    let desktop_status = app
        .oneshot(
            Request::builder()
                .uri("/v1/desktop/status")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(desktop_status.status(), StatusCode::OK);
    let desktop_body = desktop_status
        .into_body()
        .collect()
        .await
        .unwrap()
        .to_bytes();
    let desktop_payload: Value = serde_json::from_slice(&desktop_body).unwrap();
    assert_eq!(desktop_payload.as_array().unwrap().len(), 3);
}

#[tokio::test]
async fn stage2_ui_pages_are_served() {
    let app = build_router(AppState::bootstrap("secret"));

    let routes = [
        "/auth/login",
        "/auth/register",
        "/messenger",
        "/meetings/demo-room",
        "/meetings/demo-room/waiting-room",
    ];

    for route in routes {
        let response = app
            .clone()
            .oneshot(Request::builder().uri(route).body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK, "route {route}");
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let html = String::from_utf8(body.to_vec()).unwrap();
        assert!(html.contains("cdn.tailwindcss.com"), "route {route}");
    }
}
