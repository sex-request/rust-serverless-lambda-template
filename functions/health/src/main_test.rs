use super::*;

#[tokio::test]
async fn health_handle() {
    let request = Request::default();
    let expected = json!({ "message": "OK" }).into_response();
    let response = health(request, Context::default())
        .await
        .expect("expected Ok(_) value")
        .into_response();
    assert_eq!(response.body(), expected.body())
}
