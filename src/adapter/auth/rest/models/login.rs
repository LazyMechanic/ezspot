use crate::adapter::auth::rest::AccessTokenEncoded;
use crate::adapter::room::rest::RoomId;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct LoginRequest {
    pub fingerprint: String,
    pub room_id: RoomId,
    pub room_password: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct LoginResponse {
    pub access_token: AccessTokenEncoded,
}
