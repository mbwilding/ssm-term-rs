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
    use serde::{Deserialize, Serialize};
    use thiserror::Error;
    use uuid::Uuid;

    // TODO: Enum, use existing code in client
    /// InputStreamMessage represents message type for input data.
    pub const INPUT_STREAM_MESSAGE: &str = "input_stream_data";

    /// OutputStreamMessage represents message type for output data.
    pub const OUTPUT_STREAM_MESSAGE: &str = "output_stream_data";

    // AcknowledgeMessage represents message type for acknowledge.
    pub const ACKNOWLEDGE_MESSAGE: &str = "acknowledge";

    /// ChannelClosedMessage represents message type for ChannelClosed.
    pub const CHANNEL_CLOSED_MESSAGE: &str = "channel_closed";

    /// StartPublicationMessage represents the message type that notifies the CLI to start sending stream messages.
    pub const START_PUBLICATION_MESSAGE: &str = "start_publication";

    /// PausePublicationMessage represents the message type that notifies the CLI to pause sending stream messages
    /// as the remote data channel is inactive.
    pub const PAUSE_PUBLICATION_MESSAGE: &str = "pause_publication";

    /// AcknowledgeContent is used to inform the sender of an acknowledge message that the message has been received.
    /// * MessageType is a 32 byte UTF-8 string containing the message type.
    /// * MessageId is a 40 byte UTF-8 string containing the UUID identifying this message being acknowledged.
    /// * SequenceNumber is an 8 byte integer containing the message sequence number for serialized message.
    /// * IsSequentialMessage is a boolean field representing whether the acknowledged message is part of a sequence.
    #[derive(Serialize, Deserialize)]
    pub struct AcknowledgeContent {
        #[serde(rename = "AcknowledgedMessageType")]
        pub message_type: &'static str,

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
    #[derive(Serialize, Deserialize)]
    pub struct ChannelClosed {
        message_id: Uuid,
        created_date: u64,
        destination_id: String,
        session_id: String,
        message_type: String,
        schema_version: u32,
        output: String,
    }

    #[repr(u32)]
    pub enum PayloadType {
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

    #[repr(u32)]
    pub enum PayloadTypeFlag {
        DisconnectToPort = 1,
        TerminateSession = 2,
        ConnectToPortError = 3,
    }

    #[derive(Serialize, Deserialize)]
    pub struct SizeData {
        pub cols: u32,
        pub rows: u32,
    }

    pub trait IClientMessage {
        fn validate(&self) -> Result<(), ClientMessageError>;
        fn deserialize_client_message(&self, input: &[u8]) -> Result<(), ClientMessageError>;
        fn serialize_client_message(&self) -> Result<Vec<u8>, ClientMessageError>;
        fn deserialize_data_stream_acknowledge_content(&self) -> Result<AcknowledgeContent, ClientMessageError>;
        fn deserialize_channel_closed_message(&self) -> Result<ChannelClosed, ClientMessageError>;
        // TODO: fn deserialize_handshake_request(&self) -> Result<HandshakeRequestPayload, ClientMessageError>;
        // TODO: fn deserialize_handshake_complete(&self) -> Result<HandshakeCompletePayload, ClientMessageError>;
    }

    #[derive(Debug, Error)]
    pub enum ClientMessageError {
        #[error("Validation error")]
        ValidationError,

        #[error("Deserialization error")]
        DeserializationError,

        #[error("Serialization error")]
        SerializationError,

        #[error("IO error")]
        IoError(#[from] std::io::Error),
    }

    /// ClientMessage represents a message for client to send/receive. ClientMessage Message in MGS is equivalent to MDS' InstanceMessage.
    /// All client messages are sent in this form to the MGS service.
    ///
    /// * | HL|         MessageType           |Ver|  CD   |  Seq  | Flags |
    /// * |         MessageId                     |           Digest              | PayType | PayLen|
    /// * |         Payload      			|
    pub struct ClientMessage {
        /// * HL - HeaderLength is a 4 byte integer that represents the header length.
        header_length: u32,

        /// * MessageType is a 32 byte UTF-8 string containing the message type.
        message_type: String,

        /// * SchemaVersion is a 4 byte integer containing the message schema version number.
        schema_version: u32,

        /// * CreatedDate is an 8 byte integer containing the message create epoch millis in UTC.
        created_date: u64,

        /// * SequenceNumber is an 8 byte integer containing the message sequence number for serialized message streams.
        sequence_number: i64,

        /// * Flags is an 8 byte unsigned integer containing a packed array of control flags:
        /// *   Bit 0 is SYN - SYN is set (1) when the recipient should consider Seq to be the first message number in the stream
        /// *   Bit 1 is FIN - FIN is set (1) when this message is the final message in the sequence.
        flags: u64,

        /// * MessageId is a 40 byte UTF-8 string containing a random UUID identifying this message.
        message_id: Uuid,

        /// * Payload digest is a 32 byte containing the SHA-256 hash of the payload.
        payload_digest: Vec<u8>,

        /// Payload Type is a 4 byte integer containing the payload type.
        payload_type: u32,

        /// * Payload length is an 4 byte unsigned integer containing the byte length of data in the Payload field.
        payload_length: u32,

        /// * Payload is a variable length byte data.
        payload: Vec<u8>,
    }
}
