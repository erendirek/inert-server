use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use axum::{Extension, Router};

use inert::{database::create_db_pool, routes::rest::setup_rest_index_router};
use inert::utils::env_loader::{load_env_variables, EnvVars};

#[tokio::main]
async fn main() -> Result<(), &'static str>{

    let vars: EnvVars = load_env_variables();
    
    let dbp_shared_state = create_db_pool(vars.clone()).await.unwrap();
    
    let app = Router::new()
        .nest("/api/rest", setup_rest_index_router())
        .layer(Extension(dbp_shared_state.clone()))
        .layer(Extension(vars.clone()));

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    println!("listening on {}", addr);

    axum_server::Server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}