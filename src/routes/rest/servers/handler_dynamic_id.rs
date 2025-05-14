use axum::{extract::{rejection::PathRejection, Path}, response::IntoResponse, Extension, Json};
use chrono::{DateTime, Utc};
use hyper::StatusCode;
use serde::Serialize;

use crate::{database::DBPool, errors::AppError};


pub async fn get_servers_dynamic_id(server_id: Result<Path<uuid::Uuid>, PathRejection>, Extension(dbp): Extension<DBPool>) -> Result<impl IntoResponse, AppError>{

    let server_id = match server_id {
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

    let servers = match conn.query("SELECT * FROM servers WHERE id = $1", &[&server_id]).await {
        Ok(row) => row,
        Err(err) => {
            println!("{}", err);
            return Err(AppError::DatabaseError("database error".to_string()));
        },
    };

    let server = if servers.len() < 1 {
        return Err(AppError::ServerNotFound("server not found".to_string()));
    } else {
        &servers[0]
    };
    
    let server_id: uuid::Uuid = server.try_get("id").map_err(|_| AppError::InternalServerError("internal server error".to_string()))?;
    let server_name: String = server.try_get("name").map_err(|_| AppError::InternalServerError("internal server error".to_string()))?;
    let owner_id: uuid::Uuid = server.try_get("owner_id").map_err(|_| AppError::InternalServerError("internal server error".to_string()))?;
    let created_at: DateTime<Utc> = server.try_get("created_at").map_err(|_| AppError::InternalServerError("internal server error".to_string()))?;

    let server_object = ServerObject {
        server_id, server_name, owner_id, server_create_at: created_at
    };

    Ok((StatusCode::OK, Json(server_object)).into_response())
}

#[derive(Serialize)]
struct ServerObject {
    server_id: uuid::Uuid,
    server_name: String,
    owner_id: uuid::Uuid,
    server_create_at: DateTime<Utc>
}