use axum::response::IntoResponse;

pub async fn handle_index_get() -> impl IntoResponse {
    "hello world".into_response()
}