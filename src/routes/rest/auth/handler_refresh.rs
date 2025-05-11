use axum::response::IntoResponse;

pub async fn rest_handle_auth_refresh() -> impl IntoResponse {
    "hello from auth refresh"
}