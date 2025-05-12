pub mod handler_index;

use axum::{middleware, routing::{get, post}, Router};
use handler_index::{get_servers_index, post_servers_index};

use crate::middlewares::auth_middlewares::auth_required;

pub fn setup_servers_router() -> Router {
    let router = Router::new()
        .route("/", get(get_servers_index).post(post_servers_index))
        .layer(middleware::from_fn(auth_required));

    router
}