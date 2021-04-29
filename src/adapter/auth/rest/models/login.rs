use super::*;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct LoginRequest {
    pub fingerprint: String,
    pub room_id: RoomId,
    pub room_password: RoomPassword,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct LoginResponse {
    pub access_token: AccessTokenEncoded,
}
