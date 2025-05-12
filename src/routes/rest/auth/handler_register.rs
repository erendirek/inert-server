use std::{env, sync::Arc, time::{Duration, SystemTime, UNIX_EPOCH}};

use axum::{extract::rejection::JsonRejection, response::IntoResponse, Extension, Json};
use chrono::Utc;
use hyper::StatusCode;
use jsonwebtoken::{EncodingKey, Header};
use serde::{Deserialize, Serialize};
use sha2::{self, Digest};

use crate::{database::DBPool, errors::AppError, utils::{env_loader::EnvVars, jwt::JWTPayload}};

pub async fn post_auth_post_register(
    Extension(dbp): Extension<DBPool>,
    Extension(env_vars): Extension<EnvVars>,
    register_payload: Result<Json<AuthRegisterPayload>, JsonRejection>) 
        -> Result<impl IntoResponse, AppError> 
{
    let token = match register_payload {
        Ok(Json(payload)) => {
            handle_auth_register_correct_payload(&payload, &dbp, &env_vars).await?
        },

        Err(err) => {
            println!("{}", err);
            return Err(AppError::InvalidJsonType("invalid json type".to_string()));
        }
    };
    
    Ok(
        AuthRegisterResponse {
            status_code: StatusCode::OK,
            p: AuthRegisterResponseContent { msg: "register success".to_string(), token }
        }
    )
}

async fn handle_auth_register_correct_payload(auth_register_payload: &AuthRegisterPayload, dbp: &DBPool, env_vars: &EnvVars) -> Result<String, AppError> {
    let username = auth_register_payload.username.as_str();
    let email = auth_register_payload.email.as_str();
    let pwd = auth_register_payload.password.as_str();
    
    let conn = match dbp.get().await {
        Ok(val) => val,
        Err(err) => {
            println!("An error occured : {}", err);
            return Err(AppError::DatabaseError("database error".to_string()));
        }
    };


    let sh = sha2::Sha256::digest(pwd.as_bytes()).to_vec();
    let pwd_hash = hex::encode(sh);

    let user = match conn.query("INSERT INTO users (username, email, password_hash) VALUES ($1, $2, $3) RETURNING id", &[&username, &email, &pwd_hash]).await {
        Ok(val) => val,
        Err(err) => {
            println!("{}", err);
            return Err(AppError::DatabaseError("database error".to_string()));
        }
    };
    
    let user_id: uuid::Uuid = match user[0].try_get("id") {
        Ok(val) => val,
        Err(err) => {
            println!("{}", err);
            return Err(AppError::InternalServerError("internal server error".to_string()));
        },
    };

    let jwtpayload = JWTPayload {
        uuid: user_id.to_string(),
        exp: Utc::now() + Duration::from_secs(60 * 15)
    };

    let secret = env_vars.get("JWT_KEY").unwrap().as_bytes();
    let token = jsonwebtoken::encode(&Header::default(), &jwtpayload, &EncodingKey::from_secret(secret)).unwrap();
    
    Ok(token)
}

#[derive(Deserialize)]
pub struct AuthRegisterPayload {
    username: String,
    email: String,
    password: String
}

struct AuthRegisterResponse {
    status_code: StatusCode,
    p: AuthRegisterResponseContent
}

impl IntoResponse for AuthRegisterResponse {
    fn into_response(self) -> axum::response::Response {
        let status = self.status_code;
        let msg = self.p;
        (status, Json(msg)).into_response()
    }
}

#[derive(Serialize)]
struct AuthRegisterResponseContent { 
    msg: String,
    token: String 
}