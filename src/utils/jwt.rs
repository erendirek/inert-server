use serde::{Deserialize, Serialize};

pub type UserUUID = uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct JWTPayload {
    pub uuid: UserUUID,
    pub exp: u64
}