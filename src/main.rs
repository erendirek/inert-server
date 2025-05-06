use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use axum::Router;

use inert::routes::rest::setup_rest_router;

#[tokio::main]
async fn main() {

    let app = Router::new()
        .nest("/rest", setup_rest_router());

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    println!("listening on {}", addr);

    axum_server::Server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap()
}