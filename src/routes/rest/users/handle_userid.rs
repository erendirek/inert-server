use axum::{extract::{rejection::PathRejection, Path}, response::IntoResponse, Extension, Json};
use chrono::{DateTime, Utc};
use hyper::StatusCode;
use serde::Serialize;

use crate::{database::DBPool, errors::AppError};

pub async fn get_users_userid(user_id: Result<Path<uuid::Uuid>, PathRejection>, 
    Extension(dbp): Extension<DBPool>
) -> Result<impl IntoResponse, AppError> {

    let user_id = match user_id {
        Ok(Path(val)) => val,
        Err(err) => {
            println!("{}", err);
            return Err(AppError::InvalidPath("bad request".to_string()));
        },
    };

    let conn = match dbp.get().await {
        Ok(conn) => conn,
        Err(err) => {
            println!("{}", err);
            return Err(AppError::DatabaseError("database error".to_string()))
        },
    };
    
    let users = match conn.query("SELECT * FROM users WHERE id = $1", &[&user_id]).await {
        Ok(row) => row,
        Err(err) => {
            println!("{}", err);
            return Err(AppError::DatabaseError("database error".to_string()));
        },
    };

    let user = if users.len() < 1 {
        return Err(AppError::UserNotFound("user not found".to_string()));
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