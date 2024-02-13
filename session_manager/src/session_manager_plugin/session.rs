use crate::message::client_message::message::ClientMessage;
use std::any::Any;

use anyhow::Result;

pub trait ISessionPlugin {
    fn set_session_handlers(&mut self) -> Result<()>;
    fn process_stream_message_payload(
        &mut self,
        stream_data_message: ClientMessage,
    ) -> Result<bool>;
    fn initialize(&mut self, session_var: &Session);
    fn stop(&mut self);
    fn name(&self) -> &str;
}

pub trait ISession {
    fn execute(&mut self) -> Result<()>;
    fn open_data_channel(&mut self) -> Result<()>;
    fn process_first_message(&mut self, output_message: ClientMessage) -> Result<bool>;
    fn stop(&mut self);
    fn get_resume_session_params(&self) -> Result<String>;
    fn resume_session_handler(&mut self) -> Result<()>;
    fn terminate_session(&mut self) -> Result<()>;
}

pub struct Session {
    // data_channel: datachannel::IDataChannel,
    session_id: String,
    stream_url: String,
    token_value: String,
    is_aws_cli_upgrade_needed: bool,
    endpoint: String,
    client_id: String,
    target_id: String,
    sdk: Box<aws_sdk_ssm::Client>,
    // retry_params: retry::RepeatableExponentialRetryer,
    session_type: String,
    session_properties: Box<dyn Any>,
    // display_mode: sessionutil::DisplayMode,
}
