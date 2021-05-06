use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug)]
pub struct Client {
    pub id: Uuid,
    pub refresh_token: Uuid,
    pub refresh_token_exp: NaiveDateTime,
    pub fingerprint: String,
}
