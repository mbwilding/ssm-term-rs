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

use crate::config::config::{PING_TIME_INTERVAL, RETRY_ATTEMPT};
use anyhow::{bail, Result};
use bytes::Bytes;
use futures_util::{SinkExt, StreamExt};
use log::{debug, error};
use std::error::Error;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio::time::Interval;
use tokio_websockets::{MaybeTlsStream, Message, WebSocketStream};

pub trait IWebSocketChannel {
    fn initialize(&mut self, channel_url: String, channel_token: String);
    async fn open(&mut self) -> Result<()>;
    fn close(&mut self) -> Result<()>;
    async fn send_message(&mut self, message: WebSocketMessage) -> Result<()>;
    fn start_pings(&self, ping_interval: Interval);
    fn get_channel_token(&self) -> &str;
    fn get_stream_url(&self) -> &str;
    fn set_channel_token(&mut self, token: String);
    fn set_on_error(&mut self, on_error_handler: Box<dyn Fn(Box<dyn Error>)>);
    fn set_on_message(&mut self, on_message_handler: Box<dyn Fn(Vec<u8>)>);
}

struct WebSocketChannel {
    url: String,
    on_message: Arc<Mutex<Option<Box<dyn Fn(Vec<u8>)>>>>,
    on_error: Arc<Mutex<Option<Box<dyn Fn(Box<dyn Error>)>>>>,
    is_open: Arc<bool>,
    write_lock: Mutex<()>,
    connection: Option<Arc<Mutex<WebSocketStream<MaybeTlsStream<TcpStream>>>>>,
    channel_token: String,
}

impl IWebSocketChannel for WebSocketChannel {
    fn initialize(&mut self, channel_url: String, channel_token: String) {
        self.url = channel_url;
        self.channel_token = channel_token;
    }

    async fn open(&mut self) -> Result<()> {
        self.write_lock = Mutex::new(());

        let (ws, _response) = tokio_websockets::ClientBuilder::new()
            .uri(&self.url)? // TODO: Error log
            .connect()
            .await?;

        self.connection = Some(Arc::new(Mutex::new(ws)));
        self.is_open = Arc::new(true);
        self.start_pings(PING_TIME_INTERVAL);

        let is_open = Arc::clone(&self.is_open);
        let url = self.url.clone();
        let ws = Arc::clone(&self.connection.as_ref().unwrap());
        let on_message = Arc::clone(&self.on_message);
        let on_error = Arc::clone(&self.on_error);

        tokio::spawn(async move {
            let mut retry_count = 0;

            loop {
                if !*is_open {
                    debug!(
                        "Ending the channel listening routine since the channel is closed: {}",
                        &url
                    );
                    break;
                }

                let mut ws = ws.lock().await;
                match ws.next().await {
                    Some(Ok(message)) => {
                        if !message.is_binary() || !message.is_text() {
                            error!("Invalid message type. We only accept UTF-8 or binary encoded text.");
                            continue;
                        }

                        retry_count = 0;

                        if let Some(handler) = &*on_message.lock().await {
                            handler(message.as_payload().to_vec());
                        }

                        continue;
                    }
                    Some(Err(e)) => {
                        retry_count += 1;

                        if retry_count >= RETRY_ATTEMPT {
                            error!(
                                "Reached the retry limit {} for receive messages.",
                                RETRY_ATTEMPT
                            );

                            if let Some(handler) = &*on_error.lock().await {
                                handler(e.into());
                            }

                            break;
                        }

                        debug!("An error happened when receiving the message. Retried times: {}, Error: {}", retry_count, e);
                        continue;
                    }
                    None => {
                        debug!("The channel is closed: {}", &url);
                        break;
                    }
                };
            }
        });

        Ok(())
    }

    fn close(&mut self) -> Result<()> {
        Ok(())
    }

    async fn send_message(&mut self, message: WebSocketMessage) -> Result<()> {
        if !*self.is_open {
            return Ok(());
        }

        if let Some(connection) = &self.connection {
            let message = match message {
                WebSocketMessage::Binary(data) => Message::binary(Bytes::from(data)),
                WebSocketMessage::Text(data) => Message::text(data),
            };

            let mut ws = connection.lock().await;
            let _ = self.write_lock.lock().await;
            if ws.send(message).await.is_err() {
                return Ok(());
            }
        } else {
            bail!("Connection is not open");
        }

        Ok(())
    }

    fn start_pings(&self, ping_interval: Interval) {
        if let Some(connection) = &self.connection {
            let connection = Arc::clone(connection);
            let is_open = Arc::clone(&self.is_open);
            let mut ping_interval = ping_interval;

            tokio::spawn(async move {
                loop {
                    ping_interval.tick().await;

                    if !*is_open {
                        break;
                    }

                    let message = Message::ping(Bytes::from("keepalive"));
                    let mut ws = connection.lock().await;
                    if ws.send(message).await.is_err() {
                        break;
                    }
                }
            });
        }
    }

    fn get_channel_token(&self) -> &str {
        &self.channel_token
    }

    fn get_stream_url(&self) -> &str {
        &self.url
    }

    fn set_channel_token(&mut self, token: String) {
        self.channel_token = token;
    }

    fn set_on_error(&mut self, on_error_handler: Box<dyn Fn(Box<dyn Error>)>) {
        self.on_error = Some(on_error_handler);
    }

    fn set_on_message(&mut self, on_message_handler: Box<dyn Fn(Vec<u8>)>) {
        self.on_message = Some(on_message_handler);
    }
}

pub enum WebSocketMessage {
    Binary(Vec<u8>),
    Text(String),
}
