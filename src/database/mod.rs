use std::sync::Arc;

use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use postgres::{tls::NoTlsStream, Error, Socket};
use tokio_postgres::{self, Client, Connection, NoTls};

pub type DBPool = Arc<Pool<PostgresConnectionManager<NoTls>>>;

pub async fn create_db_pool() -> Result<DBPool, Error> {
    let cfg = "host=localhost user=postgres password=iamyaten dbname=inert";
    let mgr = PostgresConnectionManager::new_from_stringlike(cfg, NoTls)?;
    
    let pool = Pool::builder().build(mgr).await?;

    println!("DATABASE connection success");

    let dbp = Arc::new(pool);

    Ok(dbp)
}