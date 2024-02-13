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

use crate::config::config::MESSAGE_SCHEMA_VERSION;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct OpenDataChannelInput {
    /// MessageSchemaVersion is a required field
    #[serde(rename = "MessageSchemaVersion")]
    pub message_schema_version: &'static str,

    /// RequestId is a required field
    #[serde(rename = "RequestId")]
    pub request_id: String,

    // TokenValue is a required field
    #[serde(rename = "TokenValue")]
    pub token_value: String,

    // ClientId is a required field
    #[serde(rename = "ClientId")]
    pub client_id: String,
}

// TODO: Unofficial
impl OpenDataChannelInput {
    pub fn new(request_id: &str, token_value: &str) -> Self {
        let request_id = request_id.to_string();
        let token_value = token_value.to_string();
        let client_id = Uuid::new_v4().to_string();

        Self {
            message_schema_version: MESSAGE_SCHEMA_VERSION,
            request_id,
            token_value,
            client_id,
        }
    }
}
