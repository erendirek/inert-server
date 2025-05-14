use std::{collections::HashMap, env, sync::Arc};

use dotenv::dotenv;

pub type EnvVars = Arc<HashMap<String, String>>;

pub fn load_env_variables() -> EnvVars {
    dotenv().unwrap();

    let mut vars = HashMap::<String, String>::new();
    
    for (key, val) in env::vars() {
        vars.insert(key, val);
    }

   Arc::new(vars)
}