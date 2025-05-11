use axum::response::IntoResponse;

pub async fn rest_handle_auth_me() -> impl IntoResponse {
    "hello from auth me"
}