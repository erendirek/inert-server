use std::io;

use axum::{response::IntoResponse, Json};
use hyper::StatusCode;
use serde::Serialize;
use serde_json::json;
use thiserror::Error;

pub type ErrorCode = u32;

#[derive(Debug, Error)]
pub enum AppError {
    // 1000 class errors
    #[error("auth invalid credentials; 1001")]
    AuthInvalidCredentials(String),
    #[error("auth user not found; 1002")]
    AuthUserNotFound(String),
    #[error("auth unauthorized; 1004")]
    AuthUnauthorized(String),
    #[error("auth token expired; 1005")]
    AuthTokenExpired(String),
    
    // 6000 class errors
    #[error("internal server error; 6002")]
    InternalServerError(String),

    // 7000 class errors
    #[error["database error; 7001"]]
    DatabaseError(String),
    #[error["invalid json type; 7002"]]
    InvalidJsonType(String)
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status_code, error_code, msg) = match self {
            // 1000 class errors
            AppError::AuthInvalidCredentials(msg) => (StatusCode::UNAUTHORIZED, 1001, msg),
            AppError::AuthUserNotFound(msg) => (StatusCode::NOT_FOUND, 1002, msg),
            AppError::AuthUnauthorized(msg) => (StatusCode::UNAUTHORIZED, 1004, msg),
            AppError::AuthTokenExpired(msg) => (StatusCode::UNAUTHORIZED, 1005, msg),
            // 6000 class errors
            AppError::InternalServerError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, 6001, msg),
            // 7000 class errors
            AppError::DatabaseError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, 7001, msg),
            AppError::InvalidJsonType(msg) => (StatusCode::BAD_REQUEST, 7002, msg),
        };

        let res = json!({
            "error_code": error_code,
            "msg": msg
        });

        (status_code, Json(res)).into_response()
    }
}