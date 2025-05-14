pub mod handler_dynamic_channelid_messages;

use axum::{middleware, routing::{get, post}, Router};
use handler_dynamic_channelid_messages::{get_channels_dynamic_channelid_messages, post_channels_dynamic_channelid_messages};

use crate::middlewares::auth_middlewares::auth_required;

pub fn setup_channels_router() -> Router {
    
    let router = Router::new()
        .route("/{channel_id}/messages", post(post_channels_dynamic_channelid_messages).get(get_channels_dynamic_channelid_messages))
        .layer(middleware::from_fn(auth_required));

    router
}