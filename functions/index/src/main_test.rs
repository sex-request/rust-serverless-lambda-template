use super::*;
use lambda_http::http::Request;
use lambda_http::Body;
use serde_json::json;

#[tokio::test]
async fn without_querystring() {
    let req = Request::builder()
        .uri("http://localhost")
        .body(Body::from(()))
        .unwrap();

    assert_eq!(
        index(req, Context::default())
            .await
            .expect("expected Ok(_) value"),
        json!({})
    )
}
