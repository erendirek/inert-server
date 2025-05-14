pub mod handle_userid;

use axum::{middleware, routing::get, Router};
use handle_userid::get_users_userid;

use crate::middlewares::auth_middlewares::auth_required;

pub fn setup_users_router() -> Router {
    
    let router = Router::new()
        .route("/{user_id}", get(get_users_userid))
        .layer(middleware::from_fn(auth_required));

    router
}