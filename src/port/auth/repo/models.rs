use chrono::NaiveDateTime;
use uuid::Uuid;

pub type ClientId = Uuid;
pub type RefreshTokenSalt = Uuid;

#[derive(Debug)]
pub struct Client {
    pub id: ClientId,
    pub refresh_token_salt: RefreshTokenSalt,
    pub refresh_token_exp: NaiveDateTime,
    pub fingerprint: String,
}
