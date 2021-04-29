#[derive(serde::Deserialize, Debug)]
pub struct LoginRequest {
    pub fingerprint: String,
    pub room_id: u64,
    pub password: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct RefreshTokensRequest {
    pub fingerprint: String,
}

#[derive(serde::Serialize, Debug)]
pub struct LoginResponse {
    pub access_token: String,
}

#[derive(serde::Serialize, Debug)]
pub struct RefreshTokensResponse {
    pub access_token: String,
}

#[derive(serde::Serialize, Debug)]
pub struct WsTicket {
    pub ticket: String,
}
