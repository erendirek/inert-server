use axum::{Router, routing::get};
use crate::handlers::rest_handlers::handle_index_get;

pub fn setup_rest_router() -> Router{
    let router = Router::new()
        .route("/", get(handle_index_get));

    router
}