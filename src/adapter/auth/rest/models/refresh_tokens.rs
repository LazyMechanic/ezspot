use super::*;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct RefreshTokensRequest {
    pub fingerprint: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct RefreshTokensResponse {
    pub access_token: AccessTokenEncoded,
}
