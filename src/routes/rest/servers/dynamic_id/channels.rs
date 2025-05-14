use axum::{extract::{rejection::{JsonRejection, PathRejection}, Path}, response::IntoResponse, Extension, Json};
use chrono::{DateTime, Utc};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{database::DBPool, errors::AppError};

pub async fn get_servers_dynamic_id_channels(server_id: Result<Path<uuid::Uuid>, PathRejection>, Extension(dbp): Extension<DBPool>) -> Result<impl IntoResponse, AppError> {

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

    let channels = match conn.query("SELECT * FROM channels WHERE server_id = $1", &[&server_id]).await {
        Ok(row) => row,
        Err(err) => {
            println!("{}", err);
            return Err(AppError::DatabaseError("database error".to_string()));
        },
    };

    let mut channel_vec = Vec::<ChannelObject>::new();

    for channel in channels {
        let id: uuid::Uuid = channel.try_get("id").map_err(|_| AppError::InternalServerError("internal server error".to_string()))?;
        let server_id: uuid::Uuid = channel.try_get("server_id").map_err(|_| AppError::InternalServerError("internal server error".to_string()))?;
        let name: String = channel.try_get("name").map_err(|_| AppError::InternalServerError("internal server error".to_string()))?;
        let created_at: DateTime<Utc> = channel.try_get("created_at").map_err(|_| AppError::InternalServerError("internal server error".to_string()))?;

        channel_vec.push(ChannelObject { id, server_id, name, created_at });
    }

    let res_json = json!({
        "server_id": server_id,
        "channels": channel_vec
    });

    Ok((StatusCode::OK, Json(res_json)).into_response())
}

pub async fn post_servers_dynamic_id_channels(server_id: Result<Path<uuid::Uuid>, PathRejection>, Extension(dbp): Extension<DBPool>, req_body: Result<Json<ChannelCreateRequestBody>, JsonRejection>) -> Result<impl IntoResponse, AppError> {

    let req_body = match req_body {
        Ok(Json(req_body)) => req_body,
        Err(err) => {
            println!("{}", err);
            return Err(AppError::InvalidJsonType("invalid request body".to_string()))
        },
    };

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

    let count = match conn.execute("SELECT * FROM servers WHERE id = $1", &[&server_id]).await {
        Ok(row) => row,
        Err(err) => {
            println!("{}", err);
            return Err(AppError::DatabaseError("database error".to_string()));
        },
    };

    if count < 1 {
        return Err(AppError::ServerNotFound("server does not exist".to_string()));
    }

    let name = req_body.name;

    let _ = match conn.execute("INSERT INTO channels (server_id, name) VALUES ($1, $2)", &[&server_id, &name]).await {
        Ok(row) => row,
        Err(err) => {
            println!("{}", err);
            return Err(AppError::DatabaseError("database error".to_string()));
        },
    };

    let json_res = json!({
        "msg": format!("channel created on server {}", server_id)
    });

    Ok((StatusCode::OK, Json(json_res)).into_response())
}

#[derive(Deserialize)]
pub struct ChannelCreateRequestBody {
    name: String,
}

#[derive(Serialize)]
struct ChannelObject {
    id: uuid::Uuid,
    server_id: uuid::Uuid,
    name: String,
    created_at: DateTime<Utc>
}