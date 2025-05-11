use axum::response::IntoResponse;

pub async fn rest_handle_auth_logout() -> impl IntoResponse {
    "hello from auth logout"
}