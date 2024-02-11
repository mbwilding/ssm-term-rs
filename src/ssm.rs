use crate::enums::{EMessageType, EPayloadType};
use crate::structs::{AcknowledgeContent, AgentMessage, TermOptions};
use std::sync::Mutex;
use tracing::debug;
use uuid::Uuid;

pub fn build_init_message(term_options: TermOptions, sequence_number: i64) -> Vec<u8> {
    let init_message = AgentMessage::build_agent_message(
        &serde_json::to_string(&term_options).unwrap(),
        EMessageType::InputStreamData,
        sequence_number,
        EPayloadType::Size,
        1,
    );

    debug!("Init message: {:#?}", init_message);

    init_message.message_to_bytes()
}

pub fn build_acknowledge(sequence_number: i64, message_id: Uuid) -> Vec<u8> {
    let payload = AcknowledgeContent {
        message_type: EMessageType::OutputStreamData.to_string(),
        message_id: message_id.to_string(),
        sequence_number: sequence_number,
        is_sequential_message: true,
    };

    let json_payload = serde_json::to_string(&payload).unwrap();

    let ack_message = AgentMessage::build_agent_message(
        &json_payload,
        EMessageType::Acknowledge,
        sequence_number,
        EPayloadType::Size,
        0,
    );

    ack_message.message_to_bytes()
}

pub fn build_input_message(input: &str, sequence_number: i64) -> Vec<u8> {
    let input_message = AgentMessage::build_agent_message(
        input,
        EMessageType::InputStreamData,
        sequence_number,
        EPayloadType::Output,
        if sequence_number == 1 { 0 } else { 1 },
    );

    input_message.message_to_bytes()
}
