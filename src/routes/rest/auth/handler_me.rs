use axum::{response::IntoResponse, Extension, Json};
use chrono::{DateTime, Utc};
use hyper::StatusCode;
use serde::Serialize;

use crate::{database::DBPool, errors::AppError, utils::jwt::UserUUID};

pub async fn get_auth_me(Extension(user_uuid): Extension<UserUUID>, 
    Extension(dbp): Extension<DBPool>
) -> Result<impl IntoResponse, AppError> {

    let conn = match dbp.get().await {
        Ok(conn) => conn,
        Err(err) => {
            println!("{}", err);
            return Err(AppError::DatabaseError("database error".to_string()));
        },
    };

    let users = match conn.query("SELECT * FROM users WHERE id = ", &[&user_uuid]).await {
        Ok(users) => users,
        Err(err) => {
            println!("{}", err);
            return Err(AppError::DatabaseError("database error".to_string()));
        },
    };

    let user = if users.len() < 1 {
        return Err(AppError::AuthUnauthorized("unauthroized".to_string()));
    } else {
        &users[0]
    };

    let id: uuid::Uuid = user.try_get("id").map_err(|_| AppError::InternalServerError("internal server error".to_string()))?;
    let username: String = user.try_get("username").map_err(|_| AppError::InternalServerError("internal server error".to_string()))?;
    let email: String = user.try_get("email").map_err(|_| AppError::InternalServerError("internal server error".to_string()))?;
    let created_at: DateTime<Utc> = user.try_get("created_at").map_err(|_| AppError::InternalServerError("internal server error".to_string()))?;

    let user_object = UserObject {id, username, email, created_at};

    Ok((StatusCode::OK, Json(user_object)).into_response())
}

#[derive(Serialize)]
struct UserObject {
    id: uuid::Uuid,
    username: String,
    email: String,
    created_at: DateTime<Utc>
}