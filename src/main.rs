use crate::enums::{EMessageType, EPayloadType};
use crate::structs::{AgentMessage, Token};
use anyhow::Result;
use aws_sdk_ssm::operation::RequestId;
use aws_sdk_ssm::types::InstanceInformationStringFilter;
use bytes::Bytes;
use crossterm::event::KeyCode;
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{cursor, terminal, ExecutableCommand};
use futures_util::{SinkExt, StreamExt};
use std::io::{self, stdin, stdout, Read, Write};
use tokio::net::TcpStream;
use tokio_websockets::{MaybeTlsStream, Message, WebSocketStream};
use tracing::level_filters::LevelFilter;
use tracing::{debug, info};
use tracing_subscriber::EnvFilter;

mod enums;
mod helpers;
mod ssm;
mod structs;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::DEBUG.into())
                .from_env_lossy(),
        )
        .with_target(false)
        .without_time()
        .compact()
        .init();

    let config = aws_config::load_from_env().await;
    let ssm = aws_sdk_ssm::Client::new(&config);

    let managed_instances = ssm
        .describe_instance_information()
        //.max_results(50)
        //.filters(
        //    InstanceInformationStringFilter::builder()
        //        .key("tag:technical:product")
        //        .values("lds-terminal")
        //        .build()
        //        .unwrap(),
        //)
        .send()
        .await?;

    info!("{:?}", managed_instances);

    let instance_id = managed_instances
        .instance_information_list
        .unwrap()
        .first()
        .unwrap()
        .instance_id
        .clone()
        .unwrap();

    info!("Instance ID: {}", instance_id);

    let mut stdout = std::io::stdout();
    let mut stdin = std::io::stdin();
    terminal::enable_raw_mode()?;

    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Clear(ClearType::All))?;
    stdout.execute(cursor::MoveTo(0, 0))?;
    stdout.flush()?;

    let session = ssm
        .start_session()
        .target(instance_id)
        .reason("ssm-rs")
        .send()
        .await?;

    let (mut ws, _) = tokio_websockets::ClientBuilder::new()
        .uri(&session.stream_url.clone().unwrap())
        .unwrap()
        .connect()
        .await?;

    debug!("{:?}", ws);

    let mut sequence_number = 0_i64;

    let token = Token::build_token_message(
        session.request_id().unwrap(), //&session.session_id.unwrap(),
        &session.token_value.clone().unwrap(),
    );
    let token_json = serde_json::to_string_pretty(&token).unwrap();
    debug!("Token: {}", token_json);
    send_text(&mut ws, token_json, None).await?;

    let terminal_size = terminal::size()?;

    let term_options = structs::TermOptions {
        cols: terminal_size.0,
        rows: terminal_size.1,
    };
    let init_message = ssm::build_init_message(term_options, sequence_number);
    send_binary(&mut ws, init_message, None).await?;

    let mut input_buffer = String::new();

    loop {
        if stdin.read_to_string(&mut input_buffer)? > 0 {
            let input = ssm::build_input_message(&input_buffer, sequence_number);
            send_binary(&mut ws, input, Some(&mut sequence_number)).await?;

            input_buffer.clear();
        }

        if let Some(Ok(msg)) = ws.next().await {
            if msg.is_close() {
                break;
            }

            let bytes = msg.as_payload().iter().as_slice();
            let message = AgentMessage::bytes_to_message(bytes);

            if message.message_type != EMessageType::Acknowledge {
                let ack = ssm::build_acknowledge(sequence_number, message.message_id);
                send_binary(&mut ws, ack, None).await?;
                debug!("Sent ack for message: {:?}", message.message_id);
            }

            if message.payload_type == EPayloadType::Output {
                stdout.execute(Print(&message.payload))?;
            } else {
                debug!("{:?}", message);
            }
        }
    }

    ws.close().await?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    info!("Remote close");

    Ok(())
}

async fn send_binary(
    ws: &mut WebSocketStream<MaybeTlsStream<TcpStream>>,
    input: Vec<u8>,
    sequence_number: Option<&mut i64>,
) -> Result<()> {
    send_message(ws, Message::binary(Bytes::from(input)), sequence_number).await?;

    Ok(())
}

async fn send_text(
    ws: &mut WebSocketStream<MaybeTlsStream<TcpStream>>,
    input: String,
    sequence_number: Option<&mut i64>,
) -> Result<()> {
    send_message(ws, Message::text(input), sequence_number).await?;

    Ok(())
}

async fn send_message(
    ws: &mut WebSocketStream<MaybeTlsStream<TcpStream>>,
    input: Message,
    sequence_number: Option<&mut i64>,
) -> Result<()> {
    if let Some(sequence_number) = sequence_number {
        *sequence_number += 1;
        println!("Sequence Number: {}", sequence_number)
    }

    ws.send(input).await?;

    Ok(())
}
