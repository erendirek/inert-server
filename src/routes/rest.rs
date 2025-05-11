pub mod auth;

use auth::setup_rest_auth_router;
use axum::Router;

pub fn setup_rest_index_router() -> Router{
    let router = Router::new()
        .nest("/auth", setup_rest_auth_router());

    router
}