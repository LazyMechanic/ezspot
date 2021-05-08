use crate::port::auth::repo as auth_repo;

use chrono::NaiveDateTime;
use uuid::Uuid;

pub type ClientId = Uuid;
pub type RefreshTokenSalt = Uuid;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Client {
    pub id: ClientId,
    pub refresh_token_salt: RefreshTokenSalt,
    pub refresh_token_exp: NaiveDateTime,
    pub fingerprint: String,
}

impl From<auth_repo::Client> for Client {
    fn from(f: auth_repo::Client) -> Self {
        Self {
            id: f.id,
            refresh_token_salt: f.refresh_token_salt,
            refresh_token_exp: f.refresh_token_exp,
            fingerprint: f.fingerprint,
        }
    }
}

impl From<Client> for auth_repo::Client {
    fn from(f: Client) -> Self {
        Self {
            id: f.id,
            refresh_token_salt: f.refresh_token_salt,
            refresh_token_exp: f.refresh_token_exp,
            fingerprint: f.fingerprint,
        }
    }
}
