use axum::response::IntoResponse;

pub async fn post_auth_logout() -> impl IntoResponse {
    "hello from auth logout"
}