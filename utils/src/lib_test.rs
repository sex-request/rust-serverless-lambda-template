use super::*;
use lambda_http::http::Request;
use lambda_http::Body;
use serde_json::json;

#[test]
fn without_querystring() {
    let req = Request::builder()
        .method("GET")
        .uri("http://localhost")
        .body(Body::from(()))
        .unwrap();

    assert_eq!(query_from_req(req), json!({}))
}
