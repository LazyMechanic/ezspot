use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Entry {
    pub id: Uuid,
    pub title: String,
    pub payload: Payload,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Payload {
    pub kind: String,
    pub value: serde_json::Value,
}
