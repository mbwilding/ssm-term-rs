use crate::enums::{EMessageType, EPayloadType};
use crate::helpers::{big_endian_uuid, get_sha256_hash, pad_trim};
use byteorder::{BigEndian, ByteOrder};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

#[derive(Debug)]
pub struct AgentMessage {
    /// Header length is a 4 byte integer that represents the header length.
    pub header_length: i32,

    /// Message type is a 32 byte UTF-8 string containing the message type.
    pub message_type: EMessageType,

    /// Schema version is a 4 byte unsigned integer containing the message schema version number.
    pub schema_version: u32,

    /// Created date is an 8 byte signed integer containing the message create epoch millis in UTC.
    pub created_date: i64,

    /// SequenceNumber is an 8 byte signed integer containing the message sequence number for serialized message streams.
    pub sequence_number: i64,

    /// Flags is an 8 byte unsigned integer containing a packed array of control flags.
    ///
    /// Bit 0 is SYN - SYN it's set (1) when the recipient should consider Seq to be the first message number in the stream.
    ///
    /// Bit 1 is FIN - FIN it's set (1) when this message is the final message in the sequence.
    pub flags: u64,

    /// Message id is a 16 byte UUIDv4 identifying the message.
    pub message_id: Uuid,

    /// Payload digest is an array of 32 byte containing the SHA-256 hash of the payload.
    pub payload_digest: Vec<u8>,

    /// Payload Type is a 4 byte integer containing the payload type.
    pub payload_type: EPayloadType,

    /// Payload length is a 4 byte integer containing the byte length of data in the Payload field.
    pub payload_length: i32,

    /// Payload is a variable length string.
    pub payload: String,
}

impl AgentMessage {
    pub fn bytes_to_message(bytes: &[u8]) -> Self {
        let header_length = BigEndian::read_i32(&bytes[0..4]);
        let message_type_str = std::str::from_utf8(&bytes[4..36])
            .unwrap()
            .trim_end_matches('\0')
            .trim();
        let message_type = EMessageType::from_str(message_type_str).unwrap();

        let schema_version = BigEndian::read_u32(&bytes[36..40]);
        let created_date = BigEndian::read_i64(&bytes[40..48]);
        let sequence_number = BigEndian::read_i64(&bytes[48..56]);
        let flags = BigEndian::read_u64(&bytes[56..64]);

        let message_id = big_endian_uuid(&bytes[64..80]);

        let payload_digest = bytes[80..112].to_vec();

        let payload_type = if header_length != 112 {
            let range = (header_length - 4) as usize..header_length as usize;
            let value = BigEndian::read_i32(&bytes[range]);
            EPayloadType::from_i32(value).unwrap()
        } else {
            EPayloadType::Null
        };

        let payload_length =
            BigEndian::read_i32(&bytes[header_length as usize..(header_length + 4) as usize]);

        let payload_start = (header_length + 4) as usize;
        let payload = String::from_utf8(
            bytes[payload_start..payload_start + payload_length as usize].to_vec(),
        )
        .unwrap();

        AgentMessage {
            header_length,
            message_type,
            schema_version,
            created_date,
            sequence_number,
            flags,
            message_id,
            payload_digest,
            payload_type,
            payload_length,
            payload,
        }
    }

    pub fn message_to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        // Header Length
        let mut header_length = [0; 4];
        BigEndian::write_i32(&mut header_length, self.header_length);
        bytes.extend_from_slice(&header_length);

        // Message Type
        let message_type_bytes = self.message_type.to_string().as_bytes();
        bytes.extend_from_slice(&pad_trim(message_type_bytes, 32));

        // Schema Version
        let mut schema_version = [0; 4];
        BigEndian::write_u32(&mut schema_version, self.schema_version);
        bytes.extend_from_slice(&schema_version);

        // Created Date
        let mut created_date = [0; 8];
        BigEndian::write_i64(&mut created_date, self.created_date);
        bytes.extend_from_slice(&created_date);

        // Sequence Number
        let mut sequence_number = [0; 8];
        BigEndian::write_i64(&mut sequence_number, self.sequence_number);
        bytes.extend_from_slice(&sequence_number);

        // Flags
        let mut flags = [0; 8];
        BigEndian::write_u64(&mut flags, self.flags);
        bytes.extend_from_slice(&flags);

        // Message ID
        bytes.extend_from_slice(&self.message_id.into_bytes());

        // Payload Digest
        bytes.extend_from_slice(&self.payload_digest);

        // Payload Type
        let mut payload_type = [0; 4];
        BigEndian::write_i32(&mut payload_type, self.payload_type as i32);
        bytes.extend_from_slice(&payload_type);

        // Payload Length
        let mut payload_length = [0; 4];
        BigEndian::write_i32(&mut payload_length, self.payload_length);
        bytes.extend_from_slice(&payload_length);

        // Payload
        bytes.extend_from_slice(self.payload.as_bytes());

        bytes
    }

    pub fn build_agent_message(
        payload: &str,
        message_type: EMessageType,
        sequence_number: i64,
        payload_type: EPayloadType,
        flags: u64,
    ) -> Self {
        let payload_bytes = payload.as_bytes();
        let payload_digest = get_sha256_hash(payload);

        let created_date = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;

        Self {
            header_length: 116,
            message_type,
            schema_version: 1,
            created_date,
            sequence_number,
            flags,
            message_id: Uuid::new_v4(),
            payload_digest,
            payload_type,
            payload_length: payload_bytes.len() as i32,
            payload: payload.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TermOptions {
    pub cols: u16,
    pub rows: u16,
}

#[derive(Debug, Serialize)]
pub struct Token {
    #[serde(rename = "MessageSchemaVersion")]
    pub message_schema_version: &'static str,

    #[serde(rename = "RequestId")]
    pub request_id: String,

    #[serde(rename = "TokenValue")]
    pub token_value: String,
}

impl Token {
    pub fn build_token_message(request_id: &str, token_value: &str) -> Self {
        let request_id = request_id.to_string();
        let token_value = token_value.to_string();

        Self {
            message_schema_version: "1.0",
            request_id,
            token_value,
        }
    }
}

/// AcknowledgeContent is used to inform the sender of an acknowledge message that the message has been received.
/// * MessageType is a 32 byte UTF-8 string containing the message type.
/// * MessageId is a 40 byte UTF-8 string containing the UUID identifying this message being acknowledged.
/// * SequenceNumber is an 8 byte integer containing the message sequence number for serialized message.
/// * IsSequentialMessage is a boolean field representing whether the acknowledged message is part of a sequence
#[derive(Serialize)]
pub struct AcknowledgeContent {
    #[serde(rename = "AcknowledgedMessageType")]
    pub message_type: &'static str,

    #[serde(rename = "AcknowledgedMessageId")]
    pub message_id: String,

    #[serde(rename = "AcknowledgedMessageSequenceNumber")]
    pub sequence_number: i64,

    #[serde(rename = "IsSequentialMessage")]
    pub is_sequential_message: bool,
}
