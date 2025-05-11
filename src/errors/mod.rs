use axum::{response::IntoResponse, Json};
use hyper::StatusCode;
use serde::Serialize;
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Invalid JSON Type")]
    InvalidJSONType,

    #[error("Internal Server Error")]
    InternalServerError,

    #[error("Conflict")]
    Conflict(String),

    #[error("Dev Error")]
    DevError(String)
}

#[derive(Serialize)]
pub struct ErrorBody {
    msg: &'static str
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status_code, json_val) = match &self {
            AppError::InvalidJSONType => {
                let msg = json!({
                    "msg": "Invalid json type"
                });
                (StatusCode::BAD_REQUEST, msg)
            },
            AppError::InternalServerError => {
                let msg = json!({
                    "msg": "An unexpected error occurred. Please try again later."
                });
                (StatusCode::INTERNAL_SERVER_ERROR, msg)
            },
            AppError::Conflict(msg) => {
                let msg = json!({
                    "msg": msg
                });
                (StatusCode::CONFLICT, msg)
            },
            AppError::DevError(msg) => {
                let msg = json!({
                    "msg": msg
                });
                (StatusCode::CONFLICT, msg)
            }
        };

        let json_val = Json(json_val);
        (status_code, json_val).into_response()
    }
}