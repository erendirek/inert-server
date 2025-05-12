mod handler_register;
mod handler_login;
mod handler_logout;
mod handler_me;
mod handler_refresh;

use axum::{middleware, Router};
use axum::routing::{get, post};
use handler_login::post_auth_login;
use handler_logout::post_auth_logout;
use handler_me::get_auth_me;
use handler_refresh::post_auth_refresh;
use handler_register::post_auth_post_register;

use crate::middlewares::auth_middlewares::auth_required;

pub fn setup_auth_router() -> Router {
    let auth_router = Router::new()
        .route("/me", get(get_auth_me))
        .layer(middleware::from_fn(auth_required));

    let router = Router::new()
        .route("/login", post(post_auth_login))
        .route("/register", post(post_auth_post_register))
        .route("/logout", post(post_auth_logout))
        .route("/refresh", post(post_auth_refresh))
        .merge(auth_router);

    router
}