use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PausePublication {
    #[serde(rename = "MessageType")]
    message_type: String,

    #[serde(rename = "SchemaVersion")]
    schema_version: u32,

    #[serde(rename = "MessageId")]
    message_id: String,

    #[serde(rename = "CreateData")]
    created_date: String,
}
