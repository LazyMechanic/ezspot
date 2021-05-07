#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
pub struct GetHealthCheckResponse {
    pub msg: String,
}
