use std::str::FromStr;

use axum::{response::IntoResponse, Extension, Json};
use chrono::{DateTime, Utc};
use hyper::StatusCode;
use serde::Serialize;

use crate::{database::DBPool, errors::AppError, utils::jwt::UserUUID};

pub async fn get_servers_index(Extension(user_uuid): Extension<UserUUID>, Extension(dbp): Extension<DBPool>) -> Result<impl IntoResponse, AppError> {
    let conn = match dbp.get().await {
        Ok(val) => val,
        Err(err) => {
            println!("{}", err);
            return Err(AppError::InternalServerError("internal server error".to_string()));
        }
    };

    let user_uuid = uuid::Uuid::from_str(&user_uuid).unwrap();
    let servers = match conn.query("SELECT s.id as id, s.name as name, s.created_at as created_at 
                                        FROM server_members sm 
                                        JOIN servers s ON sm.server_id = s.id 
                                        WHERE sm.user_id = $1", &[&user_uuid])
    .await {
        Ok(res) => res,
        Err(err) =>{
            println!("{}", err);
            return Err(AppError::DatabaseError("database error".to_string()));
        },
    };

    let mut servers_vec = Vec::<ServerObject>::new();

    for server in servers {
        let id: uuid::Uuid = server.try_get("id").map_err(|err| AppError::InternalServerError("internal server error".to_string()))?;
        let name: String = server.try_get("name").map_err(|err| AppError::InternalServerError("internal server error".to_string()))?;
        let created_at: DateTime<Utc> = server.try_get("created_at").map_err(|err| AppError::InternalServerError("internal server error".to_string()))?;

        servers_vec.push(ServerObject { server_id: id.to_string(), server_name: name, server_create_at: created_at });
    }

    let servers_res = ServersIndexResponseBody {
        servers: servers_vec
    };
    
    Ok(ServersIndexResponse {
        status_code: StatusCode::OK,
        body: servers_res
    })
}

pub async fn post_servers_index() -> impl IntoResponse {
    "hello from servers post"
}

struct ServersIndexResponse {
    status_code: StatusCode,
    body: ServersIndexResponseBody
}

impl IntoResponse for ServersIndexResponse {
    fn into_response(self) -> axum::response::Response {
        (self.status_code, Json(self.body)).into_response()
    }
}

#[derive(Serialize)]
struct ServersIndexResponseBody {
    servers: Vec<ServerObject>
}

#[derive(Serialize)]
struct ServerObject {
    server_id: String,
    server_name: String,
    server_create_at: DateTime<Utc>
}