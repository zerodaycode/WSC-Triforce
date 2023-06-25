use rocket::response::{self};

use dotenvy_macro::dotenv;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::response::Responder;
use rocket::{Request, Response};
use serde_json::json;

use rocket::http::ContentType;

use std::io::Cursor as SyncCursor;

pub struct ApiKey(String);
pub struct ApiKeyError {
    message: String,
    status: Status,
}
pub enum ApiKeyResult {
    Ok(ApiKey),
    Err(ApiKeyError),
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKeyResult {
    type Error = std::convert::Infallible;

    async fn from_request(
        request: &'r Request<'_>,
    ) -> rocket::request::Outcome<ApiKeyResult, Self::Error> {
        let keys: Vec<_> = request.headers().get("x-api-key").collect();
        match keys.len() {
            0 => Outcome::Success(ApiKeyResult::Err(ApiKeyError {
                message: "Missing x-api-key".into(),
                status: Status::BadRequest,
            })),
            1 if keys[0] == option_env!("API_KEY").unwrap_or(dotenv!("API_KEY")) => {
                Outcome::Success(ApiKeyResult::Ok(ApiKey(keys[0].to_string())))
            }
            1 => Outcome::Success(ApiKeyResult::Err(ApiKeyError {
                message: "Invalid x-api-key".into(),
                status: Status::Unauthorized,
            })),
            _ => Outcome::Success(ApiKeyResult::Err(ApiKeyError {
                message: "Multiple x-api-keys".into(),
                status: Status::BadRequest,
            })),
        }
    }
}

impl<'r> Responder<'r, 'static> for ApiKeyError {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let mut response = Response::new();
        response.set_status(self.status);
        response.set_header(ContentType::JSON);

        let body = json!({ "error": self.message }).to_string();
        let cursor = SyncCursor::new(body.into_bytes());

        response.set_sized_body(None, cursor);

        Ok(response)
    }
}
