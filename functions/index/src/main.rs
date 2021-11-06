use lambda_http::lambda_runtime::Error;
use lambda_http::{handler, lambda_runtime, Context, Request};
use serde_json::Value;
use utils::query_from_req;

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(handler(index)).await?;
    Ok(())
}

async fn index(req: Request, _: Context) -> Result<Value, Error> {
    return Ok(query_from_req(req));
}

#[cfg(test)]
mod main_test;
