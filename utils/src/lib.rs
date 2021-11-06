use lambda_http::{Request, RequestExt};
use serde_json::{json, Value};
use std::collections::HashMap;

pub fn query_from_req(req: Request) -> Value {
    let query_params = req.query_string_parameters();
    let mut query: HashMap<String, String> = HashMap::new();

    for (key, value) in query_params.iter() {
        query.insert(String::from(key), String::from(value));
    }
    return json!(query);
}

#[cfg(test)]
mod lib_test;
