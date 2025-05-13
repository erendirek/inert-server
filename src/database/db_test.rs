use postgres::NoTls;

use crate::utils::env_loader::EnvVars;


pub async fn db_test(env_vars: EnvVars) {
    let postgre_name = env_vars.get("RDS_DB_NAME").unwrap();
    let postgre_username = env_vars.get("RDS_DB_USERNAME").unwrap();
    let postgre_password = env_vars.get("RDS_DB_PASSWORD").unwrap();
    let postgre_domain = env_vars.get("RDS_DB_SERVER_DOMAIN").unwrap();

    let cfg = format!("postgresql://{}:{}@{}:{}/{}", 
                        postgre_username, postgre_password, postgre_domain, 5432, postgre_name);
    let conn = tokio_postgres::connect(&cfg, NoTls).await.expect("DB connection error");

    println!("DB CONNECTIOB SUCCESS")
}