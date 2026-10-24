use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use mandu_server::{create_app, AppState, ServerConfig};
use serde_json::Value;
use std::fs;
use tempfile::tempdir;
use tower::ServiceExt;

#[tokio::test]
async fn test_health_check() {
    let tmp = tempdir().unwrap();
    let config = ServerConfig {
        vault_path: tmp.path().to_path_buf(),
        auth_logout_url: "https://auth.exemplo.com/logout".to_string(),
        ..Default::default()
    };
    let state = AppState::new(config);
    let app = create_app(state);

    let req = Request::builder()
        .uri("/health")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body).expect("deve ser JSON válido");
    assert_eq!(json["status"], "ok");
}

#[tokio::test]
async fn test_config_endpoint() {
    let tmp = tempdir().unwrap();
    let config = ServerConfig {
        vault_path: tmp.path().to_path_buf(),
        auth_logout_url: "https://auth.custom.com/logout".to_string(),
        ..Default::default()
    };
    let state = AppState::new(config);
    let app = create_app(state);

    let req = Request::builder()
        .uri("/api/config")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["auth_logout_url"], "https://auth.custom.com/logout");
}

#[tokio::test]
async fn test_tree_and_notes_crud() {
    let tmp = tempdir().unwrap();
    let note1_path = tmp.path().join("nota1.md");
    fs::write(&note1_path, "# Minha Primeira Nota\nConteudo da nota").unwrap();

    let config = ServerConfig {
        vault_path: tmp.path().to_path_buf(),
        ..Default::default()
    };
    let state = AppState::new(config);
    let app = create_app(state);

    // 1. GET /api/tree
    let req = Request::builder()
        .uri("/api/tree")
        .body(Body::empty())
        .unwrap();
    let response = app.clone().oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();
    assert!(json.as_array().unwrap().len() >= 1);

    // 2. GET /api/notes?path=nota1.md
    let req = Request::builder()
        .uri("/api/notes?path=nota1.md")
        .body(Body::empty())
        .unwrap();
    let response = app.clone().oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["relative_path"], "nota1.md");
    assert!(json["content"]
        .as_str()
        .unwrap()
        .contains("Minha Primeira Nota"));

    // 3. PUT /api/notes?path=nota_salva.md
    let req = Request::builder()
        .method("PUT")
        .uri("/api/notes?path=nota_salva.md")
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::to_vec(&serde_json::json!({
                "content": "# Nota Nova\nCriada via API"
            }))
            .unwrap(),
        ))
        .unwrap();
    let response = app.clone().oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert!(tmp.path().join("nota_salva.md").exists());

    // 4. Path traversal bloqueado
    let req = Request::builder()
        .uri("/api/notes?path=../../etc/passwd")
        .body(Body::empty())
        .unwrap();
    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
