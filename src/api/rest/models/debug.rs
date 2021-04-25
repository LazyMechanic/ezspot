#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
pub struct PostRequest {
    pub string_field: String,
    pub int_field: i64,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
pub struct PostResponse {
    pub string_field: String,
    pub int_field: i64,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
pub struct GetRequest {
    pub string_field: String,
    pub int_field: i64,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
pub struct GetErrorRequest {
    pub kind: ErrorKind,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
pub struct GetResponse {
    pub string_field: String,
    pub int_field: i64,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum ErrorKind {
    StructError,
    TupleError,
}

impl ToString for ErrorKind {
    fn to_string(&self) -> String {
        match *self {
            ErrorKind::StructError => "struct-error".to_owned(),
            ErrorKind::TupleError => "tuple-error".to_owned(),
        }
    }
}

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum Error {
    #[error("Struct error: string_field={string_field}, int_field={int_field}")]
    StructError {
        string_field: String,
        int_field: i64,
    },
    #[error("Tuple error: string_field={0}, int_field={1}")]
    TupleError(String, i64),
}
