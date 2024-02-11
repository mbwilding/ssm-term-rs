use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StartPublication {
    #[serde(rename = "MessageId")]
    message_id: String,

    #[serde(rename = "CreatedDate")]
    created_date: String,

    #[serde(rename = "DestinationId")]
    destination_id: String,

    #[serde(rename = "SessionId")]
    session_id: String,

    #[serde(rename = "MessageType")]
    message_type: String,

    #[serde(rename = "SchemaVersion")]
    schema_version: u32,

    #[serde(rename = "Output")]
    output: String,
}
