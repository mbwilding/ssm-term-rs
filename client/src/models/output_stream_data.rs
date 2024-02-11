use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct OutputStreamData {
    #[serde(rename = "AgentVersion")]
    agent_version: String,

    #[serde(rename = "RequestedClientActions")]
    requested_client_actions: Vec<RequestedClientAction>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestedClientAction {
    #[serde(rename = "ActionType")]
    action_type: String,

    #[serde(rename = "ActionParameters")]
    action_parameters: ActionParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionParameters {
    #[serde(rename = "SessionType")]
    session_type: Option<String>,

    #[serde(rename = "Properties")]
    properties: Option<Vec<String>>,

    #[serde(rename = "KMSKeyId")]
    kms_key_id: Option<String>,
}
