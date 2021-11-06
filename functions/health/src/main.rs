use lambda_http::lambda_runtime::Error;
use lambda_http::{handler, lambda_runtime, Context, IntoResponse, Request};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(handler(health)).await?;
    Ok(())
}

async fn health(_: Request, _: Context) -> Result<impl IntoResponse, Error> {
    // `serde_json::Values` impl `IntoResponse` by default
    // creating an application/json response
    Ok(json!({ "message": "OK" }))
}

#[cfg(test)]
mod main_test;
