use axum::response::IntoResponse;

pub async fn rest_handle_auth_login() -> impl IntoResponse {
    "hello from rest handle auth login"
}