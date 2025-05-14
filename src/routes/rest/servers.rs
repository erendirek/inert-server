pub mod handler_index;
pub mod handler_dynamic_id;
pub mod dynamic_id;

use axum::{middleware, routing::get, Router};
use dynamic_id::channels::{get_servers_dynamic_id_channels, post_servers_dynamic_id_channels};
use handler_dynamic_id::get_servers_dynamic_id;
use handler_index::{get_servers_index, post_servers_index};

use crate::middlewares::auth_middlewares::auth_required;

pub fn setup_servers_router() -> Router {
    
    let router = Router::new()
        .route("/", get(get_servers_index).post(post_servers_index))
        .route("/{server_id}", get(get_servers_dynamic_id))
        .route("/{server_id}/channels", get(get_servers_dynamic_id_channels).post(post_servers_dynamic_id_channels))
        .layer(middleware::from_fn(auth_required));

    router
}