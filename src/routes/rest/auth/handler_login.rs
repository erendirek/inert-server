use std::time::{Duration, SystemTime, UNIX_EPOCH};

use axum::{extract::rejection::JsonRejection, response::IntoResponse, Extension, Json};
use chrono::{DateTime, Utc};
use hyper::StatusCode;
use jsonwebtoken::{EncodingKey, Header};
use serde::{Deserialize, Serialize};
use sha2::Digest;

use crate::{database::DBPool, errors::AppError, utils::{env_loader::EnvVars, jwt::JWTPayload}};

pub async fn post_auth_login(Extension(dbp): Extension<DBPool>, Extension(env_vars): Extension<EnvVars>, register_payload: Result<Json<AuthLoginReqBody>, JsonRejection>) -> Result<impl IntoResponse, AppError> {
    
    let res = match register_payload {
        Ok(Json(payload)) => {
            handle_auth_login_correct_payload(&payload, &dbp, &env_vars).await?
        },
        Err(err) => {
            println!("{}", err);
            return Err(AppError::InvalidJsonType("invalid json type".to_string()))
        }
    };

    Ok(AuthLoginResponse {
        status_code: StatusCode::OK,
        p: res
    })
}

async fn handle_auth_login_correct_payload(payload: &AuthLoginReqBody, dbp: &DBPool, env_vars: &EnvVars) -> Result<AuthLoginResponseContent, AppError> {

    let username = payload.username.as_str();
    let password = payload.password.as_str();

    let conn = match dbp.get().await {
        Ok(conn) => conn,
        Err(err) => {
            println!("{}", err);
            return Err(AppError::DatabaseError("database error".to_string()));
        },
    };

    let user = match conn.query("SELECT id, password_hash FROM users WHERE username=$1", &[&username]).await {
        Ok(val) => val,
        Err(err) => {
            println!("{}", err);
            return Err(AppError::DatabaseError("database error".to_string()));
        },
    };
    
    let user = if user.len() <= 0 {
        return Err(AppError::AuthUserNotFound("user does not exist".to_string()));
    } else {
        &user[0]
    };

    let pwd_hash: String = match user.try_get("password_hash") {
        Ok(val) => val,
        Err(err) => {
            println!("{}", err);
            return Err(AppError::InternalServerError("internal server error".to_string()));
        },
    };

    let user_id: uuid::Uuid = match user.try_get("id") {
        Ok(val) => val,
        Err(err) => {
            println!("{}", err);
            return Err(AppError::InternalServerError("internal server error".to_string()));
        },
    };

    let sh = sha2::Sha256::digest(password.as_bytes()).to_vec();
    let hashed_pass = hex::encode(sh);

    let jwtpayload = JWTPayload {
        uuid: user_id.to_string(),
        // exp: (Utc::now() + Duration::from_secs(60 * 15)).signed_duration_since(Utc)
        exp: SystemTime::now().duration_since(UNIX_EPOCH).expect("time backwards").as_millis() + Duration::from_secs(60 * 15).as_millis()
    };
    
    let secret = env_vars.get("JWT_KEY").unwrap().as_bytes();
    let token = jsonwebtoken::encode(&Header::default(), &jwtpayload, &EncodingKey::from_secret(secret)).unwrap();

    if pwd_hash == hashed_pass {
        Ok(AuthLoginResponseContent {
            msg: "login success".to_string(),
            token
        })
    } else {
        Ok(AuthLoginResponseContent {
            msg: "login failed".to_string(),
            token: "".to_string()
        })
    }
}

#[derive(Deserialize)]
pub struct AuthLoginReqBody {
    username: String,
    password: String,
}

struct AuthLoginResponse {
    status_code: StatusCode,
    p: AuthLoginResponseContent
}

impl IntoResponse for AuthLoginResponse {
    fn into_response(self) -> axum::response::Response {
        let status = self.status_code;
        let p = self.p;
        (status, Json(p)).into_response()
    }
}

#[derive(Serialize)]
struct AuthLoginResponseContent { 
    msg: String,
    token: String
}