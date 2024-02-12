// Copyright 2018 Amazon.com, Inc. or its affiliates. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License"). You may not
// use this file except in compliance with the License. A copy of the
// License is located at
//
// http://aws.amazon.com/apache2.0/
//
// or in the "license" file accompanying this file. This file is distributed
// on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND,
// either express or implied. See the License for the specific language governing
// permissions and limitations under the License.

/// message package defines data channel messages structure.
pub mod message {
    use serde::{Serialize, Deserialize};
    use std::time::Duration;
    use strum_macros::Display;

    /// ActionType used in Handshake to determine action requested by the agent
    #[derive(Serialize, Deserialize, Display, Debug)]
    pub enum ActionType {
        KMSEncryption,
        SessionType,
    }

    /// This is used in Handshake to determine status of the action requested by the agent.
    #[derive(Serialize, Deserialize, Display, Debug)]
    pub enum ActionStatus {
        Success = 1,
        Failed = 2,
        Unsupported = 3,
    }

    /// This is sent by the agent to initialize KMS encryption.
    #[derive(Serialize, Deserialize, Debug)]
    pub struct KMSEncryptionRequest {
        #[serde(rename = "KMSKeyId")]
        pub kms_key_id: String,
    }

    /// This is received by the agent to set up KMS encryption.
    #[derive(Serialize, Deserialize, Debug)]
    pub struct KMSEncryptionResponse {
        #[serde(rename = "KMSCipherTextKey")]
        pub kms_cipher_text_key: Vec<u8>,
        #[serde(rename = "KMSCipherTextHash")]
        pub kms_cipher_text_hash: Vec<u8>,
    }

    /// SessionType request contains type of the session that needs to be launched and properties for plugin.
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "PascalCase")]
    pub struct SessionTypeRequest {
        pub session_type: String,
        pub properties: serde_json::Value,
    }

    /// Handshake payload sent by the agent to the session manager plugin.
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "PascalCase")]
    pub struct HandshakeRequestPayload {
        pub agent_version: String,
        pub requested_client_actions: Vec<RequestedClientAction>,
    }

    /// An action requested by the agent to the plugin.
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "PascalCase")]
    pub struct RequestedClientAction {
        pub action_type: ActionType,
        pub action_parameters: serde_json::Value,
    }

    /// The result of processing the action by the plugin.
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "PascalCase")]
    pub struct ProcessedClientAction {
        pub action_type: ActionType,
        pub action_status: ActionStatus,
        pub action_result: serde_json::Value,
        pub error: String,
    }

    /// Handshake Response sent by the plugin in response to the handshake request.
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "PascalCase")]
    pub struct HandshakeResponsePayload {
        pub client_version: String,
        pub processed_client_actions: Vec<ProcessedClientAction>,
        pub errors: Vec<String>,
    }

    /// This is sent by the agent as a challenge to the client. The challenge field
    /// is some data that was encrypted by the agent. The client must be able to decrypt
    /// this and in turn encrypt it with its own key.
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "PascalCase")]
    pub struct EncryptionChallengeRequest {
        pub challenge: Vec<u8>,
    }

    /// This is received by the agent from the client. The challenge field contains
    /// some data received, decrypted and then encrypted by the client. Agent must
    /// be able to decrypt this and verify it matches the original plaintext challenge.
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "PascalCase")]
    pub struct EncryptionChallengeResponse {
        pub challenge: Vec<u8>,
    }

    /// Handshake Complete indicates to client that handshake is complete.
    /// This signals the client to start the plugin and display a customer message where appropriate.
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "PascalCase")]
    pub struct HandshakeCompletePayload {
        pub handshake_time_to_complete: Duration,
        pub customer_message: String,
    }
}
