use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct QueueResposne {
    pub name: String,
    pub messages: i64,
}
