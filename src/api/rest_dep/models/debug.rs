use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Debug)]
pub struct PostRequest {
    pub string_field: String,
    pub int_field: i64,
}

#[derive(Deserialize, Debug)]
pub struct PostResponse {
    pub string_field: String,
    pub int_field: i64,
}

#[derive(Serialize, Debug)]
pub struct GetResponse {
    pub string_field: String,
    pub int_field: i64,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Second error: string_field={string_field}, int_field={int_field:?}")]
    Error {
        string_field: String,
        int_field: i64,
    },
}
