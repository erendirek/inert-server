use chrono::Utc;
use serde::{Deserialize, Serialize};

pub type UserUUID = String;

#[derive(Serialize, Deserialize, Debug)]
pub struct JWTPayload {
    pub uuid: String,
    pub exp: u64
}