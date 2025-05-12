pub mod auth;
pub mod servers;

use auth::setup_auth_router;
use axum::Router;
use servers::setup_servers_router;

pub fn setup_rest_index_router() -> Router {
    let router = Router::new()
        .nest("/auth", setup_auth_router())
        .nest("/servers", setup_servers_router());

    router
}