use axum::{response::IntoResponse, Error, Json};
use hyper::StatusCode;
use serde_json::{json, Value};

pub type ErrorCode = u32;

pub const AUTH_INVALID_CREDENTIALS: ErrorCode = 1001;
pub const AUTH_USER_NOT_FOUND: ErrorCode = 1002;
pub const AUTH_USER_ALREADY_EXISTS: ErrorCode = 1003;
pub const AUTH_UNAUTHORIZED: ErrorCode = 1004;

pub const USER_NOT_FOUND: ErrorCode = 2001;
pub const USER_ALREADY_FRIEND: ErrorCode = 2002;
pub const USER_BLOCKED: ErrorCode = 2003;

pub const CHANNEL_NOT_FOUND: ErrorCode = 3001;
pub const CHANNEL_ACCESS_DENIED: ErrorCode = 3002;
pub const CHANNEL_ALREADY_EXISTS: ErrorCode = 3003;

pub const MESSAGE_NOT_FOUND: ErrorCode = 4001;
pub const MESSAGE_SEND_FAILED: ErrorCode = 4002;
pub const MESSAGE_TOO_LONG: ErrorCode = 4003;

pub const SERVER_NOT_FOUND: ErrorCode = 5001;
pub const SERVER_ACCESS_DENIED: ErrorCode = 5002;

pub const DATABASE_ERROR: ErrorCode = 6001;
pub const INVALID_JSON_TYPE: ErrorCode = 6002;

pub const VALIDATION_FAILED: ErrorCode = 7001;
pub const INTERNAL_SERVER_ERROR: ErrorCode = 7002;
pub const RATE_LIMITED: ErrorCode = 7003;

pub struct AppError {
    status_code: StatusCode,
    response: Value
}

impl AppError {
    pub fn cerror(code: ErrorCode, msg: String) -> AppError {
        let status_code = AppError::get_status(&code);
        let response = json!({
            "code": code,
            "msg": msg
        });
    
        AppError { status_code, response }
    }

    fn get_status(code: &ErrorCode) -> StatusCode{
        match code {
            _ => StatusCode::OK
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (self.status_code, Json(self.response)).into_response()
    }
}