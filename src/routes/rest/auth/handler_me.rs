use axum::{response::IntoResponse, Extension};

use crate::utils::jwt::UserUUID;

pub async fn get_auth_me(Extension(user_uuid): Extension<UserUUID>) -> impl IntoResponse {
    format!("hello from auth me {}", user_uuid)
}