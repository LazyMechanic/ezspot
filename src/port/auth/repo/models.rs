use chrono::NaiveDateTime;
use uuid::Uuid;

pub type ClientId = Uuid;
pub type RefreshTokenSalt = Uuid;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Client {
    pub id: ClientId,
    pub refresh_token_salt: RefreshTokenSalt,
    pub refresh_token_exp: NaiveDateTime,
    pub fingerprint: String,
}
