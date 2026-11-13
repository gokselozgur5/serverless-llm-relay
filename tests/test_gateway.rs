use axum::body::{to_bytes, Body};
use axum::http::{Request, StatusCode};
use serde_json::Value;
use serverless_llm_relay::gateway::create_app;
use tower::ServiceExt;

async fn body_json(res: axum::response::Response) -> Value {
    let bytes = to_bytes(res.into_body(), usize::MAX).await.unwrap();
    serde_json::from_slice(&bytes).unwrap()
}

#[tokio::test]
async fn test_health_returns_ok() {
    let req = Request::get("/health").body(Body::empty()).unwrap();
    let res = create_app().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(body_json(res).await["status"], "ok");
}

#[tokio::test]
async fn test_telegram_webhook_accepts_update() {
    let req = Request::post("/webhook/telegram")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"update_id":42,"message":{"text":"hi"}}"#))
        .unwrap();
    let res = create_app().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(body_json(res).await["ok"], true);
}
