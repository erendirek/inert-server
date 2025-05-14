use std::sync::Arc;

use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use native_tls::TlsConnector;
use postgres::Error;
use postgres_native_tls::MakeTlsConnector;

use crate::utils::env_loader::EnvVars;

pub type DBPool = Arc<Pool<PostgresConnectionManager<MakeTlsConnector>>>;

pub async fn create_db_pool(env_vars: EnvVars) -> Result<DBPool, Error> {
    
    let postgre_name = env_vars.get("RDS_DB_NAME").unwrap();
    let postgre_username = env_vars.get("RDS_DB_USERNAME").unwrap();
    let postgre_password = env_vars.get("RDS_DB_PASSWORD").unwrap();
    let postgre_domain = env_vars.get("RDS_DB_SERVER_DOMAIN").unwrap();

    let connector = TlsConnector::builder().danger_accept_invalid_certs(true).build().unwrap();
    let connector = MakeTlsConnector::new(connector);

    let conn_string = format!("host={} user={} password={} dbname={} port=5432 sslmode=require", postgre_domain, postgre_username, postgre_password, postgre_name);

    let mgr = PostgresConnectionManager::new_from_stringlike(conn_string, connector)?;
    
    let pool = Pool::builder().build(mgr).await?;
    
    let dbp = Arc::new(pool);

    let dbp_clone = dbp.clone();

    match dbp_clone.get().await {
        Ok(val) => {},
        Err(err) => {
            println!("{}", err);
        },
    }
    
    println!("DATABASE connection success");


    Ok(dbp)
}