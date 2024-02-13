use crate::communicator::web_sockets_channel::IWebSocketChannel;
use crate::encryption::encrypter::Encrypter;
use crate::message::client_message::message::ClientMessage;
use anyhow::Result;
use std::any::Any;
use std::collections::{HashMap, LinkedList};
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};

struct DataChannel {
    ws_channel: Arc<dyn IWebSocketChannel>,
    role: String,
    client_id: String,
    session_id: String,
    target_id: String,
    is_aws_cli_upgrade_needed: bool,

    /// records sequence number of last acknowledged message received over data channel
    expected_sequence_number: i64,

    /// Records sequence number of last stream data message sent over data channel
    stream_data_sequence_number: i64,

    /// Buffer to store outgoing stream messages until acknowledged
    /// using linked list for this buffer as access to oldest message is required and it support faster deletion from any position of list
    outgoing_message_buffer: ListMessageBuffer<StreamingMessage>,

    /// Buffer to store incoming stream messages if received out of sequence
    /// using map for this buffer as incoming messages can be out of order and retrieval would be faster by sequenceId
    incoming_message_buffer: MapMessageBuffer,

    // Round trip time of latest acknowledged message
    round_trip_time: f64,

    /// Round trip time variation of latest acknowledged message
    round_trip_time_variation: f64,

    /// Timeout used for resending unacknowledged message
    retransmission_timeout: Duration,

    /// Encrypter to encrypt/decrypt if agent requests encryption
    encryption: Encrypter,
    encryption_enabled: bool,

    /// SessionType
    session_type: String,
    is_session_type_set: Mutex<bool>,
    session_properties: Box<dyn Any>,

    /// Used to detect if resending a streaming message reaches timeout
    is_stream_message_resend_timeout: Mutex<bool>,

    /// Handles data on output stream. Output stream is data outputted by the SSM agent and received here.
    output_stream_handlers: Vec<OutputStreamDataMessageHandler>,
    is_session_specific_handler_set: bool,

    /// AgentVersion received during handshake
    agent_version: String,
}

struct ListMessageBuffer<T> {
    messages: Mutex<LinkedList<T>>,
    capacity: usize,
}

struct MapMessageBuffer {
    messages: Mutex<HashMap<i64, StreamingMessage>>,
}

struct StreamingMessage {
    content: Vec<u8>,
    sequence_number: i64,
    last_sent_time: SystemTime,
    resend_attempt: Option<i32>,
}

type OutputStreamDataMessageHandler = Box<dyn Fn(ClientMessage) -> Result<bool> + Send + Sync>;
