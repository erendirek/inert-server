use axum::response::IntoResponse;

pub async fn helper_handler() -> impl IntoResponse {
    "It Works!!!".into_response()
}