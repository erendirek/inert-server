use std::{sync::Arc, time::{SystemTime, UNIX_EPOCH}};

use axum::{extract::rejection::JsonRejection, response::IntoResponse, Extension, Json};
use hex::encode;
use hyper::StatusCode;
use postgres::error::SqlState;
use serde::{Deserialize, Serialize};
use sha2::{self, Digest};

use crate::{database::DBPool, errors::AppError};

pub async fn rest_handle_auth_post_register(
    Extension(dbp): Extension<Arc<DBPool>>, 
    login_payload: Result<Json<AuthRegisterPayload>, JsonRejection>) 
        -> Result<impl IntoResponse, AppError> 
{
    match login_payload {
        Ok(Json(payload)) => {
            handle_auth_register_correct_payload(&payload, &dbp).await?;
        },

        Err(err) => {
            return Err(AppError::InvalidJSONType);
        }
    }

    Ok(LoginResponse {
        status_code: StatusCode::OK,
        msg: "success"
    })
}

async fn handle_auth_register_correct_payload(auth_register_payload: &AuthRegisterPayload, dbp: &Arc<DBPool>) -> Result<(), AppError> {
    let username = auth_register_payload.username.as_str();
    let email = auth_register_payload.email.as_str();
    let pwd = auth_register_payload.password.as_str();
    
    let conn = match dbp.get().await {
        Ok(val) => val,
        Err(err) => {
            println!("An error occured : {}", err);
            return Err(AppError::InternalServerError);
        }
    };


    let sh = sha2::Sha256::digest(pwd.as_bytes()).to_vec();
    let pwd_hash = encode(sh);

    let res = match conn.query("INSERT INTO users (username, email, pwd_hash) VALUES ($1, $2, $3) RETURNING id", &[&username, &email, &pwd_hash]).await {
        Ok(val) => val,
        Err(err) => {
            println!("An error occured : {}", err);
            return Err(handle_auth_register_db_error(err));
        }
    };
    
    let id: uuid::Uuid = res[0].get("id");
    
    let myc = JWTClaims {
        uuid: id.to_string(),
        exp: SystemTime::now().duration_since(UNIX_EPOCH).expect("time backwards").as_millis()
    };
    
    Ok(())
}

fn handle_auth_register_db_error(err: postgres::Error) -> AppError{
    if let Some(sql_state) = err.code() {
        match *sql_state {
            SqlState::UNIQUE_VIOLATION => return AppError::Conflict(err.to_string()),
            _ => return AppError::InternalServerError
        }
    }

    AppError::InternalServerError
}

#[derive(Deserialize)]
pub struct AuthRegisterPayload {
    username: String,
    email: String,
    password: String
}

#[derive(Serialize, Deserialize, Debug)]
struct JWTClaims {
    uuid: String,
    exp: u128
}

struct LoginResponse {
    status_code: StatusCode,
    msg: &'static str
}

impl IntoResponse for LoginResponse {
    fn into_response(self) -> axum::response::Response {
        let status = self.status_code;
        let msg = self.msg;
        (status, msg).into_response()
    }
}