
use axum::{extract::Request, middleware::Next, response::Response, Extension};
use chrono::Utc;
use hyper::HeaderMap;
use jsonwebtoken::{DecodingKey, TokenData, Validation};

use crate::{errors::AppError, utils::{env_loader::EnvVars, jwt::{JWTPayload, UserUUID}}};

pub async fn auth_required(env_vars: Extension<EnvVars>, mut req: Request, next: Next) -> Result<Response, AppError> {

    let secret = env_vars.get("JWT_KEY").unwrap();
    let token = extract_token(req.headers());

    let token = if let Some(val) = token {
        val
    } else {
        return Err(AppError::AuthUnauthorized("token does not exist".to_string()));
    };

    let token_claims = if let Some(token_data) = token_is_valid(token, &secret) {
        token_data.claims
    } else {
        return Err(AppError::AuthUnauthorized("invalid token".to_string()));
    };

    if Utc::now().timestamp_millis().unsigned_abs() > token_claims.exp {
        return Err(AppError::AuthTokenExpired("token expired".to_string()));
    }

    req.extensions_mut().insert(token_claims.uuid as UserUUID);

    let res = next.run(req).await;
    Ok(res)
}

fn extract_token(headers: &HeaderMap) -> Option<String> {
    headers.get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .map(|s| s.to_string())
}

fn token_is_valid(token: String, secret: &String) -> Option<TokenData<JWTPayload>> {    
    return match jsonwebtoken::decode::<JWTPayload>(token.as_str(), &DecodingKey::from_secret(secret.as_bytes()), &Validation::default()) {
        Ok(val) => Some(val),
        Err(err) => {
            println!("{}", err);
            return None;
        },
    };
}