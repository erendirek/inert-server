use std::collections::HashMap;

use axum::{extract::{rejection::{JsonRejection, PathRejection, QueryRejection}, Path, Query}, response::IntoResponse, Extension, Json};
use chrono::{DateTime, Utc};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{database::DBPool, errors::AppError, utils::jwt::UserUUID};

pub async fn get_channels_dynamic_channelid_messages(Extension(user_uuid): Extension<UserUUID>, 
    Extension(dbp): Extension<DBPool>, 
    channel_id: Result<Path<uuid::Uuid>, PathRejection>,
    query_params: Result<Query<HashMap<String,u32>>, QueryRejection>
) -> Result<impl IntoResponse, AppError> {

    let mut has_query_params = false;
    let query_params = match query_params {
        Ok(Query(val)) => {
            has_query_params = true;
            val
        },
        Err(_) => HashMap::new(),
    };

    let page = if has_query_params && query_params.contains_key("page") {
        query_params.get("page").unwrap().clone()
    } else { 0 };

    let channel_id = match channel_id {
        Ok(Path(val)) => val,
        Err(err) => {
            println!("{}", err);
            return Err(AppError::InvalidPath("bad request".to_string()));
        },
    };

    let conn = match dbp.get().await {
        Ok(val) => val,
        Err(err) => {
            println!("{}", err);
            return Err(AppError::InternalServerError("internal server error".to_string()));
        }
    };

    let count = match conn.execute("SELECT * FROM channels WHERE id = $1", &[&channel_id]).await {
        Ok(c) => c,
        Err(err) => {
            println!("{}", err);
            return Err(AppError::DatabaseError("database error".to_string()))
        },
    };

    if count < 1 {
        return Err(AppError::ChannelNotFound("channel does not exist".to_string()));
    }

    let limit: i64 = 20;
    let offset = limit * page as i64;
    let messages = match conn.query("SELECT m.id AS id, channel_id, author_id, content, m.created_at AS created_at, m.edited_at AS edited_at, deleted, username, email 
	    FROM messages m
	    INNER JOIN users u ON u.id = m.author_id 
	    WHERE channel_id = $1
	    ORDER BY m.created_at DESC
        LIMIT $2 OFFSET $3", 
    &[&channel_id, &limit, &offset]).await {
        Ok(val) => val,
        Err(err) => {
            println!("{}", err);
            return Err(AppError::DatabaseError("database error".to_string()))
        },
    };
    
    let mut messages_vec = Vec::<MessageObject>::new();
    for message in messages {
        let id: uuid::Uuid = message.try_get("id").map_err(|_| AppError::InternalServerError("internal server error1".to_string()))?;
        let channel_id: uuid::Uuid = message.try_get("channel_id").map_err(|_| AppError::InternalServerError("internal server error2".to_string()))?;
        let author_id: uuid::Uuid = message.try_get("author_id").map_err(|_| AppError::InternalServerError("internal server error3".to_string()))?;
        let content: String = message.try_get("content").map_err(|_| AppError::InternalServerError("internal server error4".to_string()))?;
        let created_at: DateTime<Utc> = message.try_get("created_at").map_err(|_| AppError::InternalServerError("internal server error5".to_string()))?;
        let edited_at: Option<DateTime<Utc>> = message.try_get("edited_at").map_err(|_| AppError::InternalServerError("internal server error6".to_string()))?;
        let deleted: bool = message.try_get("deleted").map_err(|_| AppError::InternalServerError("internal server error6".to_string()))?;
        let username: String = message.try_get("username").map_err(|_| AppError::InternalServerError("internal server error6".to_string()))?;
        let email: String = message.try_get("email").map_err(|_| AppError::InternalServerError("internal server error6".to_string()))?;

        messages_vec.push(MessageObject { id, channel_id, author_id, content, created_at, edited_at, deleted, username, email });
    }

    let res_json = json!({
        "channel_id": channel_id,
        "messages": messages_vec
    });

    Ok((StatusCode::OK, Json(res_json)).into_response())
}

pub async fn post_channels_dynamic_channelid_messages(Extension(user_uuid): Extension<UserUUID>, 
    Extension(dbp): Extension<DBPool>, 
    channel_id: Result<Path<uuid::Uuid>, PathRejection>, 
    req_body: Result<Json<SendMessageRequestBody>, JsonRejection>
) -> Result<impl IntoResponse, AppError> {

    let req_body = match req_body {
        Ok(Json(val)) => val,
        Err(err) => {
            println!("{}", err);
            return Err(AppError::InvalidJsonType("invalid request body".to_string()));
        },
    };

    let channel_id = match channel_id {
        Ok(Path(val)) => val,
        Err(err) => {
            println!("{}", err);
            return Err(AppError::InvalidPath("bad request".to_string()));
        },
    };

    let conn = match dbp.get().await {
        Ok(val) => val,
        Err(err) => {
            println!("{}", err);
            return Err(AppError::InternalServerError("internal server error".to_string()));
        }
    };

    let count = match conn.execute("SELECT * FROM channels WHERE id = $1", &[&channel_id]).await {
        Ok(c) => c,
        Err(err) => {
            println!("{}", err);
            return Err(AppError::DatabaseError("database error".to_string()))
        },
    };

    if count < 1 {
        return Err(AppError::ChannelNotFound("channel does not exist".to_string()));
    }

    let content = req_body.content;
    let _ = match conn.execute("INSERT INTO messages (channel_id, author_id, content) VALUES ($1, $2, $3)", &[&channel_id, &user_uuid, &content]).await {
        Ok(val) => val,
        Err(err) => {
            println!("{}", err);
            return Err(AppError::DatabaseError("database error".to_string()))
        },
    };
    
    let json_res = json!({
        "msg": format!("message created on channel {}", channel_id)
    });

    Ok((StatusCode::OK, Json(json_res)).into_response())
}

#[derive(Deserialize)]
pub struct SendMessageRequestBody {
    content: String,
}

#[derive(Serialize)]
struct MessageObject {
    id: uuid::Uuid,
    channel_id: uuid::Uuid,
    author_id: uuid::Uuid,
    content: String,
    created_at: DateTime<Utc>,
    edited_at: Option<DateTime<Utc>>,
    deleted: bool,
    username: String,
    email: String,
}