use uuid::Uuid;

#[derive(Debug)]
pub struct Entry {
    pub id: Uuid,
    pub title: String,
    pub payload: Payload,
}

#[derive(Debug)]
pub struct Payload {
    pub kind: String,
    pub value: serde_json::Value,
}
