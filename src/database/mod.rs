pub mod db_test;

use std::{env, fs, io::BufReader, path::{Path, PathBuf}, sync::Arc};

use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use postgres::{tls::{MakeTlsConnect, NoTlsStream, TlsConnect}, Error, Socket};
use rustls::{ClientConfig, RootCertStore};
use tokio_postgres::{self, Client, Connection, NoTls};
use tokio_postgres_rustls::MakeRustlsConnect;

use crate::{errors::AppError, utils::env_loader::EnvVars};

pub type DBPool = Arc<Pool<PostgresConnectionManager<MakeRustlsConnect>>>;

pub async fn create_db_pool(env_vars: EnvVars) -> Result<DBPool, Error> {
    
    let current_dir: String = env::current_dir().unwrap().display().to_string();

    let postgre_name = env_vars.get("RDS_DB_NAME").unwrap();
    let postgre_username = env_vars.get("RDS_DB_USERNAME").unwrap();
    let postgre_password = env_vars.get("RDS_DB_PASSWORD").unwrap();
    let postgre_domain = env_vars.get("RDS_DB_SERVER_DOMAIN").unwrap();

    let mut path = PathBuf::from(current_dir);
    path.push("rds-ca.pem");
    let path = path.display().to_string();
    let mut reader = BufReader::new(path.as_bytes());

    let mut root_store = RootCertStore::empty();
    root_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
    let certs = rustls_pemfile::certs(&mut reader);
    for cert in certs {
        root_store.add(cert.unwrap()).unwrap();
    }

    let config = ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    let rustls_connect = MakeRustlsConnect::new(config);

    let cfg = format!("postgresql://{}:{}@{}:{}/{}?sslmode=require", postgre_username, postgre_password, postgre_domain, 5432, postgre_name);

    let mgr = PostgresConnectionManager::new_from_stringlike(cfg, rustls_connect)?;
    
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