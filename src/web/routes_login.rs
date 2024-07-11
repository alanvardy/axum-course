use crate::{Error, Result};
use axum::Json;
use serde::Deserialize;
use serde_json::{json, Value};

async fn api_login(payload: Json<LoginPayload>) -> Result<Json<Value>> {
    // TODO implement read db/auth logic

    if payload.username != "demo1" || payload.pwd != "welcome" {
        return Err(Error::LoginFail);
    };

    // TODO set cookies

    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}
