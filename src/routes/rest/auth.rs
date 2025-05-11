mod handler_register;
mod handler_login;
mod handler_logout;
mod handler_me;
mod handler_refresh;

use axum::Router;
use axum::routing::{get, post};
use handler_login::rest_handle_auth_login;
use handler_logout::rest_handle_auth_logout;
use handler_me::rest_handle_auth_me;
use handler_refresh::rest_handle_auth_refresh;
use handler_register::rest_handle_auth_post_register;

pub fn setup_rest_auth_router() -> Router {
    let router = Router::new()
        .route("/login", post(rest_handle_auth_login))
        .route("/register", post(rest_handle_auth_post_register))
        .route("/logout", post(rest_handle_auth_logout))
        .route("/refresh", post(rest_handle_auth_refresh))
        .route("/me", get(rest_handle_auth_me));

    router
}