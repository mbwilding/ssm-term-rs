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

/// Message package defines data channel messages structure.
pub mod message {
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Serialize};
    use strum_macros::{AsRefStr, Display, EnumString};
    use thiserror::Error;
    use uuid::Uuid;

    /// MessageType represents the type of message.
    #[derive(Serialize, Deserialize, EnumString, AsRefStr, Display, Debug, PartialEq)]
    #[strum(serialize_all = "snake_case")]
    #[serde(rename_all = "snake_case")]
    pub enum MessageType {
        /// InteractiveShell message type for interactive shell.
        InteractiveShell,

        /// TaskReply represents message type for task reply.
        AgentTaskReply,

        /// TaskComplete represents message type for task complete.
        AgentTaskComplete,

        /// TaskAcknowledge represents message type for acknowledge of tasks sent over control channel.
        AgentTaskAcknowledge,

        /// AcknowledgeMessage represents message type for acknowledge.
        Acknowledge,

        /// AgentSessionState represents status of session.
        AgentSessionState,

        /// ChannelClosedMessage represents message type for ChannelClosed.
        ChannelClosed,

        /// OutputStreamMessage represents message type for output data.
        OutputStreamData,

        /// InputStreamMessage represents message type for input data.
        InputStreamData,

        /// PausePublicationMessage represents the message type that notifies the CLI to pause sending stream messages
        /// as the remote data channel is inactive.
        PausePublication,

        /// StartPublicationMessage represents the message type that notifies the CLI to start sending stream messages.
        StartPublication,

        /// AgentJob represents message type for agent job.
        AgentJob,

        /// AgentJobAcknowledge represents message for agent job acknowledge.
        AgentJobAck,

        /// AgentJobReplyAck represents message for agent job reply acknowledge.
        AgentJobReplyAck,

        /// AgentJobReply represents message type for agent job reply.
        AgentJobReply,
    }

    /// AcknowledgeContent is used to inform the sender of an acknowledge message that the message has been received.
    /// * MessageType is a 32 byte UTF-8 string containing the message type.
    /// * MessageId is a 40 byte UTF-8 string containing the UUID identifying this message being acknowledged.
    /// * SequenceNumber is an 8 byte integer containing the message sequence number for serialized message.
    /// * IsSequentialMessage is a boolean field representing whether the acknowledged message is part of a sequence.
    #[derive(Serialize, Deserialize, Debug)]
    pub struct AcknowledgeContent {
        #[serde(rename = "AcknowledgedMessageType")]
        pub message_type: MessageType,

        #[serde(rename = "AcknowledgedMessageId")]
        pub message_id: Uuid,

        #[serde(rename = "AcknowledgedMessageSequenceNumber")]
        pub sequence_number: i64,

        #[serde(rename = "IsSequentialMessage")]
        pub is_sequential_message: bool,
    }

    /// ChannelClosed is used to inform the client to close the channel.
    /// * MessageId is a 40 byte UTF-8 string containing the UUID identifying this message.
    /// * CreatedDate is a string field containing the message create epoch millis in UTC.
    /// * DestinationId is a string field containing the session target.
    /// * SessionId is a string field representing which session to close.
    /// * MessageType is a 32 byte UTF-8 string containing the message type.
    /// * SchemaVersion is a 4 byte integer containing the message schema version number.
    /// * Output is a string field containing the error message for channel close.
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "PascalCase")]
    pub struct ChannelClosed {
        message_id: Uuid,
        created_date: String,
        destination_id: String,
        session_id: String,
        message_type: String,
        schema_version: i32,
        output: String,
    }

    #[derive(Display, Copy, Clone, PartialEq, Debug)]
    #[repr(u32)]
    pub enum PayloadType {
        Null = 0,
        Output = 1,
        Error = 2,
        Size = 3,
        Parameter = 4,
        HandshakeRequestPayloadType = 5,
        HandshakeResponsePayloadType = 6,
        HandshakeCompletePayloadType = 7,
        EncChallengeRequest = 8,
        EncChallengeResponse = 9,
        Flag = 10,
        StdErr = 11,
        ExitCode = 12,
    }

    impl From<PayloadType> for u32 {
        fn from(value: PayloadType) -> Self {
            match value {
                PayloadType::Null => 0,
                PayloadType::Output => 1,
                PayloadType::Error => 2,
                PayloadType::Size => 3,
                PayloadType::Parameter => 4,
                PayloadType::HandshakeRequestPayloadType => 5,
                PayloadType::HandshakeResponsePayloadType => 6,
                PayloadType::HandshakeCompletePayloadType => 7,
                PayloadType::EncChallengeRequest => 8,
                PayloadType::EncChallengeResponse => 9,
                PayloadType::Flag => 10,
                PayloadType::StdErr => 11,
                PayloadType::ExitCode => 12,
            }
        }
    }

    impl From<u32> for PayloadType {
        fn from(value: u32) -> Self {
            match value {
                0 => PayloadType::Null,
                1 => PayloadType::Output,
                2 => PayloadType::Error,
                3 => PayloadType::Size,
                4 => PayloadType::Parameter,
                5 => PayloadType::HandshakeRequestPayloadType,
                6 => PayloadType::HandshakeResponsePayloadType,
                7 => PayloadType::HandshakeCompletePayloadType,
                8 => PayloadType::EncChallengeRequest,
                9 => PayloadType::EncChallengeResponse,
                10 => PayloadType::Flag,
                11 => PayloadType::StdErr,
                12 => PayloadType::ExitCode,
                _ => panic!("Invalid value for PayloadType: {}", value),
            }
        }
    }

    #[derive(Display, Debug)]
    #[repr(u32)]
    pub enum PayloadTypeFlag {
        DisconnectToPort = 1,
        TerminateSession = 2,
        ConnectToPortError = 3,
    }

    #[derive(Serialize, Debug)]
    pub struct SizeData {
        pub cols: u32,
        pub rows: u32,
    }

    pub trait IClientMessage {
        fn validate(&self) -> Result<(), ClientMessageError>;
        fn deserialize_client_message(&self, input: &[u8]) -> Result<(), ClientMessageError>;
        fn serialize_client_message(&self) -> Result<Vec<u8>, ClientMessageError>;
        fn deserialize_data_stream_acknowledge_content(
            &self,
        ) -> Result<AcknowledgeContent, ClientMessageError>;
        fn deserialize_channel_closed_message(&self) -> Result<ChannelClosed, ClientMessageError>;
        // TODO: fn deserialize_handshake_request(&self) -> Result<HandshakeRequestPayload, ClientMessageError>;
        // TODO: fn deserialize_handshake_complete(&self) -> Result<HandshakeCompletePayload, ClientMessageError>;
    }

    #[derive(Error, Debug)]
    pub enum ClientMessageError {
        #[error("Validation error")]
        ValidationError(String),

        #[error("Deserialization error")]
        DeserializationError(String),

        #[error("Serialization error")]
        SerializationError(String),

        #[error("IO error")]
        IoError(#[from] std::io::Error),
    }

    /// ClientMessage represents a message for client to send/receive. ClientMessage Message in MGS is equivalent to MDS' InstanceMessage.
    /// All client messages are sent in this form to the MGS service.
    ///
    /// * | HL|         MessageType           |Ver|  CD   |  Seq  | Flags |
    /// * |         MessageId                     |           Digest              | PayType | PayLen|
    /// * |         Payload      			|
    #[derive(Debug)]
    pub struct ClientMessage {
        /// * HL - HeaderLength is a 4 byte unsigned integer that represents the header length.
        pub header_length: u32,

        /// * MessageType is a 32 byte UTF-8 string containing the message type.
        pub message_type: MessageType,

        /// * SchemaVersion is a 4 byte unsigned integer containing the message schema version number.
        pub schema_version: u32,

        /// * CreatedDate is an 8 byte unsigned integer containing the message create epoch millis in UTC.
        pub created_date: DateTime<Utc>,

        /// * SequenceNumber is an 8 byte signed integer containing the message sequence number for serialized message streams.
        pub sequence_number: i64,

        /// * Flags is an 8 byte unsigned integer containing a packed array of control flags:
        /// *   Bit 0 is SYN - SYN is set (1) when the recipient should consider Seq to be the first message number in the stream
        /// *   Bit 1 is FIN - FIN is set (1) when this message is the final message in the sequence.
        pub flags: u64,

        /// * MessageId is a 40 byte UTF-8 string containing a random UUID identifying this message.
        pub message_id: Uuid,

        /// * Payload digest is a 32 byte containing the SHA-256 hash of the payload.
        pub payload_digest: Vec<u8>,

        /// Payload Type is a 4 byte unsigned integer containing the payload type.
        pub payload_type: PayloadType,

        /// * Payload length is an 4 byte unsigned integer containing the byte length of data in the Payload field.
        pub payload_length: u32,

        /// * Payload is a variable length byte data.
        pub payload: String,
    }

    impl ClientMessage {
        pub const HL_LENGTH: usize = 4;
        pub const MESSAGE_TYPE_LENGTH: usize = 32;
        pub const SCHEMA_VERSION_LENGTH: usize = 4;
        pub const CREATED_DATE_LENGTH: usize = 8;
        pub const SEQUENCE_NUMBER_LENGTH: usize = 8;
        pub const FLAGS_LENGTH: usize = 8;
        pub const MESSAGE_ID_LENGTH: usize = 16;
        pub const PAYLOAD_DIGEST_LENGTH: usize = 32;
        pub const PAYLOAD_TYPE_LENGTH: usize = 4;
        pub const PAYLOAD_LENGTH_LENGTH: usize = 4;

        pub const HL_OFFSET: usize = 0;
        pub const MESSAGE_TYPE_OFFSET: usize = Self::HL_OFFSET + Self::HL_LENGTH;
        pub const SCHEMA_VERSION_OFFSET: usize =
            Self::MESSAGE_TYPE_OFFSET + Self::MESSAGE_TYPE_LENGTH;
        pub const CREATED_DATE_OFFSET: usize =
            Self::SCHEMA_VERSION_OFFSET + Self::SCHEMA_VERSION_LENGTH;
        pub const SEQUENCE_NUMBER_OFFSET: usize =
            Self::CREATED_DATE_OFFSET + Self::CREATED_DATE_LENGTH;
        pub const FLAGS_OFFSET: usize = Self::SEQUENCE_NUMBER_OFFSET + Self::SEQUENCE_NUMBER_LENGTH;
        pub const MESSAGE_ID_OFFSET: usize = Self::FLAGS_OFFSET + Self::FLAGS_LENGTH;
        pub const PAYLOAD_DIGEST_OFFSET: usize = Self::MESSAGE_ID_OFFSET + Self::MESSAGE_ID_LENGTH;
        pub const PAYLOAD_TYPE_OFFSET: usize =
            Self::PAYLOAD_DIGEST_OFFSET + Self::PAYLOAD_DIGEST_LENGTH;
        pub const PAYLOAD_LENGTH_OFFSET: usize =
            Self::PAYLOAD_TYPE_OFFSET + Self::PAYLOAD_TYPE_LENGTH;
        pub const PAYLOAD_OFFSET: usize = Self::PAYLOAD_LENGTH_OFFSET + Self::PAYLOAD_LENGTH_LENGTH;
    }
}
