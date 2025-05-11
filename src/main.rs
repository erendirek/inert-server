use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use axum::{Extension, Router};

use dotenv::dotenv;
use inert::{database::create_db_pool, routes::rest::setup_rest_index_router};

#[tokio::main]
async fn main() -> Result<(), &'static str>{
    // Load ENV
    dotenv().unwrap();

    let dbp_shared_state = create_db_pool().await.unwrap();
    
    let app = Router::new()
        .nest("/rest", setup_rest_index_router())
        .layer(Extension(dbp_shared_state));

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    println!("listening on {}", addr);

    axum_server::Server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}