use axum::response::IntoResponse;

pub async fn post_auth_refresh() -> impl IntoResponse {
    "hello from auth refresh"
}