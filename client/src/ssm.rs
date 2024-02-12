use crate::helpers::get_sha256_hash;
use session_manager::message::client_message::message::{
    AcknowledgeContent, ClientMessage, MessageType, PayloadType, SizeData,
};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::debug;
use uuid::Uuid;

pub fn build_init_message(term_options: SizeData, sequence_number: i64) -> Vec<u8> {
    let init_message = build_agent_message(
        serde_json::to_string(&term_options).unwrap(),
        MessageType::InputStreamData,
        sequence_number,
        PayloadType::Size,
        1,
    );

    debug!("Init message: {:#?}", init_message);

    init_message.serialize_client_message()
}

pub fn build_acknowledge(sequence_number: i64, message_id: Uuid) -> Vec<u8> {
    let payload = AcknowledgeContent {
        message_type: MessageType::OutputStreamData,
        message_id,
        sequence_number,
        is_sequential_message: true,
    };

    let ack_message = build_agent_message(
        serde_json::to_string(&payload).unwrap(),
        MessageType::Acknowledge,
        sequence_number,
        PayloadType::Size,
        0,
    );

    ack_message.serialize_client_message()
}

#[allow(dead_code)]
pub fn build_input_message(input: String, sequence_number: i64) -> Vec<u8> {
    let input_message = build_agent_message(
        input,
        MessageType::InputStreamData,
        sequence_number,
        PayloadType::Output,
        if sequence_number == 1 { 0 } else { 1 },
    );

    input_message.serialize_client_message()
}

fn build_agent_message(
    payload: String,
    message_type: MessageType,
    sequence_number: i64,
    payload_type: PayloadType,
    flags: u64,
) -> ClientMessage {
    let payload_bytes = payload.as_bytes();
    let payload_digest = get_sha256_hash(&payload);

    let created_date = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;

    ClientMessage {
        header_length: 116,
        message_type,
        schema_version: 1,
        created_date,
        sequence_number,
        flags,
        message_id: Uuid::new_v4(),
        payload_digest,
        payload_type,
        payload_length: payload_bytes.len() as u32,
        payload: payload,
    }
}
