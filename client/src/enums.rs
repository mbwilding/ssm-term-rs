use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EPayloadType {
    Null = 0,
    Output = 1,
    Error = 2,
    Size = 3,
    Parameter = 4,
    HandshakeRequest = 5,
    HandshakeResponse = 6,
    HandshakeComplete = 7,
    EncChallengeRequest = 8,
    EncChallengeResponse = 9,
    Flag = 10,
    StdErr = 11,
    ExitCode = 12,
}

impl EPayloadType {
    pub fn from_i32(value: i32) -> Option<EPayloadType> {
        match value {
            0 => Some(EPayloadType::Null),
            1 => Some(EPayloadType::Output),
            2 => Some(EPayloadType::Error),
            3 => Some(EPayloadType::Size),
            4 => Some(EPayloadType::Parameter),
            5 => Some(EPayloadType::HandshakeRequest),
            6 => Some(EPayloadType::HandshakeResponse),
            7 => Some(EPayloadType::HandshakeComplete),
            8 => Some(EPayloadType::EncChallengeRequest),
            9 => Some(EPayloadType::EncChallengeResponse),
            10 => Some(EPayloadType::Flag),
            11 => Some(EPayloadType::StdErr),
            12 => Some(EPayloadType::ExitCode),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum EMessageType {
    /// InteractiveShell message type for interactive shell
    InteractiveShell,

    /// TaskReply represents message type for task reply
    TaskReply,

    /// TaskComplete represents message type for task complete
    TaskComplete,

    /// TaskAcknowledge represents message type for acknowledge of tasks sent over control channel
    TaskAcknowledge,

    /// Acknowledge represents message type for acknowledge
    Acknowledge,

    /// AgentSessionState represents status of session
    AgentSession,

    /// ChannelClosed represents message type for ChannelClosed
    ChannelClosed,

    /// OutputStreamData represents message type for outgoing stream data
    OutputStreamData,

    /// InputStreamData represents message type for incoming stream data
    InputStreamData,

    /// PausePublication message type for pause sending data packages
    PausePublication,

    /// StartPublication message type for start sending data packages
    StartPublication,

    /// AgentJob represents message type for agent job
    AgentJob,

    /// AgentJobAcknowledge represents message for agent job acknowledge
    AgentJobAcknowledge,

    /// AgentJobReplyAck represents message for agent job reply acknowledge
    AgentJobReplyAck,

    /// AgentJobReply represents message type for agent job reply
    AgentJobReply,
}

impl EMessageType {
    /// Returns a string slice representing the message type.
    pub fn to_string(&self) -> &str {
        match self {
            EMessageType::InteractiveShell => "interactive_shell",
            EMessageType::TaskReply => "agent_task_reply",
            EMessageType::TaskComplete => "agent_task_complete",
            EMessageType::TaskAcknowledge => "agent_task_acknowledge",
            EMessageType::Acknowledge => "acknowledge",
            EMessageType::AgentSession => "agent_session_state",
            EMessageType::ChannelClosed => "channel_closed",
            EMessageType::OutputStreamData => "output_stream_data",
            EMessageType::InputStreamData => "input_stream_data",
            EMessageType::PausePublication => "pause_publication",
            EMessageType::StartPublication => "start_publication",
            EMessageType::AgentJob => "agent_job",
            EMessageType::AgentJobAcknowledge => "agent_job_ack",
            EMessageType::AgentJobReplyAck => "agent_job_reply_ack",
            EMessageType::AgentJobReply => "agent_job_reply",
        }
    }
}

impl FromStr for EMessageType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "interactive_shell" => Ok(EMessageType::InteractiveShell),
            "agent_task_reply" => Ok(EMessageType::TaskReply),
            "agent_task_complete" => Ok(EMessageType::TaskComplete),
            "agent_task_acknowledge" => Ok(EMessageType::TaskAcknowledge),
            "acknowledge" => Ok(EMessageType::Acknowledge),
            "agent_session_state" => Ok(EMessageType::AgentSession),
            "channel_closed" => Ok(EMessageType::ChannelClosed),
            "output_stream_data" => Ok(EMessageType::OutputStreamData),
            "input_stream_data" => Ok(EMessageType::InputStreamData),
            "pause_publication" => Ok(EMessageType::PausePublication),
            "start_publication" => Ok(EMessageType::StartPublication),
            "agent_job" => Ok(EMessageType::AgentJob),
            "agent_job_ack" => Ok(EMessageType::AgentJobAcknowledge),
            "agent_job_reply_ack" => Ok(EMessageType::AgentJobReplyAck),
            "agent_job_reply" => Ok(EMessageType::AgentJobReply),
            _ => Err(()),
        }
    }
}
