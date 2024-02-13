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

use std::time::Duration;
use tokio::time::Interval;

pub const ROLE_PUBLISH_SUBSCRIBE: &str = "publish_subscribe";
pub const MESSAGE_SCHEMA_VERSION: &str = "1.0";
pub const DEFAULT_TRANSMISSION_TIMEOUT: Duration = Duration::from_millis(200);
pub const DEFAULT_ROUND_TRIP_TIME: Duration = Duration::from_millis(100);
pub const DEFAULT_ROUND_TRIP_TIME_VARIATION: u32 = 0;
pub const RESEND_SLEEP_INTERVAL: Duration = Duration::from_millis(100);
pub const RESEND_MAX_ATTEMPT: u32 = 3000; // 5 minutes / ResendSleepInterval
pub const STREAM_DATA_PAYLOAD_SIZE: usize = 1024;
pub const OUTGOING_MESSAGE_BUFFER_CAPACITY: usize = 10000;
pub const INCOMING_MESSAGE_BUFFER_CAPACITY: usize = 10000;
pub const RTT_CONSTANT: f32 = 1.0 / 8.0; // Round trip time constant
pub const RTTV_CONSTANT: f32 = 1.0 / 4.0; // Round trip time variation constant
pub const CLOCK_GRANULARITY: Duration = Duration::from_millis(10);
pub const MAX_TRANSMISSION_TIMEOUT: Duration = Duration::from_secs(1);
pub const RETRY_BASE: u32 = 2;
pub const DATA_CHANNEL_NUM_MAX_RETRIES: u32 = 5;
pub const DATA_CHANNEL_RETRY_INITIAL_DELAY_MILLIS: u64 = 100;
pub const DATA_CHANNEL_RETRY_MAX_INTERVAL_MILLIS: u64 = 5000;
pub const RETRY_ATTEMPT: u32 = 5;
pub const PING_TIME_INTERVAL: Interval = tokio::time::interval(Duration::from_secs(60 * 5)); // 5 minutes

// Plugin names
pub const SHELL_PLUGIN_NAME: &str = "Standard_Stream";
pub const PORT_PLUGIN_NAME: &str = "Port";
pub const INTERACTIVE_COMMANDS_PLUGIN_NAME: &str = "InteractiveCommands";
pub const NON_INTERACTIVE_COMMANDS_PLUGIN_NAME: &str = "NonInteractiveCommands";

// Agent Versions
pub const TERMINATE_SESSION_FLAG_SUPPORTED_AFTER_THIS_AGENT_VERSION: &str = "2.3.722.0";
pub const TCP_MULTIPLEXING_SUPPORTED_AFTER_THIS_AGENT_VERSION: &str = "3.0.196.0";
pub const TCP_MULTIPLEXING_WITH_SMUX_KEEP_ALIVE_DISABLED_AFTER_THIS_AGENT_VERSION: &str =
    "3.1.1511.0";
