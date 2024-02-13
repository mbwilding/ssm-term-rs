use crate::models::channel_closed::ChannelClosed;
use crate::models::pause_publication::PausePublication;
use anyhow::Result;
use aws_sdk_ssm::operation::RequestId;
use bytes::Bytes;
use crossterm::terminal;
use futures_util::{SinkExt, StreamExt};
use session_manager::message::client_message::message::{
    ClientMessage, MessageType, PayloadType, SizeData,
};
use session_manager::service::service::OpenDataChannelInput;
use tokio::io::{self, AsyncWriteExt, Stdout};
use tokio::net::TcpStream;
use tokio_websockets::{MaybeTlsStream, Message, WebSocketStream};
use tracing::level_filters::LevelFilter;
use tracing::{debug, info};
use tracing_subscriber::EnvFilter;

mod helpers;
mod models;
mod ssm;

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

    terminal::enable_raw_mode()?;

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

    //stdout.execute(EnterAlternateScreen)?;
    //stdout.execute(Clear(ClearType::All))?;
    //stdout.execute(cursor::MoveTo(0, 0))?;
    //stdout.flush()?;

    let session = ssm
        .start_session()
        .target(instance_id)
        .reason("ssm-rs")
        .send()
        .await?;

    let (mut ws, _response) = tokio_websockets::ClientBuilder::new()
        .uri(&session.stream_url.clone().unwrap())
        .unwrap()
        .connect()
        .await?;

    info!("Connected");

    debug!("{:?}", ws);

    #[allow(unused_mut)]
    let mut sequence_number = 0_i64;

    let token = OpenDataChannelInput::new(
        session.request_id().unwrap(),
        &session.token_value.clone().unwrap(),
    );
    let token_json = serde_json::to_string(&token).unwrap();
    debug!("Token: {}", token_json);
    send_text(&mut ws, token_json).await?;

    let terminal_size = terminal::size()?;

    let size_data = SizeData {
        cols: terminal_size.0 as u32,
        rows: terminal_size.1 as u32,
    };
    let init_message = ssm::build_init_message(size_data, sequence_number);
    send_binary(&mut ws, init_message, None).await?;
    //send_binary(&mut ws, init_message, Some(&mut sequence_number)).await?;

    let mut stdout = io::stdout();

    loop {
        //if stdin.poll_read(&mut input_buffer).await? > 0 {
        //    let input = ssm::build_input_message(&input_buffer, sequence_number);
        //    send_binary(&mut ws, input, Some(&mut sequence_number)).await?;
        //    input_buffer.clear();
        //}

        if let Some(Ok(msg)) = ws.next().await {
            if msg.is_close() {
                break;
            }

            let bytes = msg.as_payload().iter().as_slice();
            let message = ClientMessage::deserialize_client_message(bytes)?;

            println!(
                "Payload [{}]\n{}",
                &message.message_type.to_string(),
                &message.payload
            );

            match message.message_type {
                MessageType::InteractiveShell => {}
                MessageType::AgentTaskReply => {}
                MessageType::AgentTaskComplete => {}
                MessageType::AgentTaskAcknowledge => {}
                MessageType::Acknowledge => {
                    //send_binary(
                    //    &mut ws,
                    //    ssm::build_input_message("ls\n".to_string(), sequence_number),
                    //    Some(&mut sequence_number),
                    //)
                    //.await?;
                    continue;
                }
                MessageType::AgentSessionState => {}
                MessageType::ChannelClosed => {
                    let payload = serde_json::from_str::<ChannelClosed>(&message.payload).unwrap();
                    println!("{:#?}", &payload);
                }
                MessageType::OutputStreamData => {
                    // TODO
                    //let payload =
                    //    serde_json::from_str::<OutputStreamData>(&message.payload).unwrap();
                    //println!("{:#?}", &payload);
                }
                MessageType::InputStreamData => {}
                MessageType::PausePublication => {
                    let payload =
                        serde_json::from_str::<PausePublication>(&message.payload).unwrap();
                    println!("{:#?}", &payload);
                }
                MessageType::StartPublication => {
                    println!("StartPublication: {:?}", &message.payload);
                }
                MessageType::AgentJob => {}
                MessageType::AgentJobAck => {}
                MessageType::AgentJobReplyAck => {}
                MessageType::AgentJobReply => {}
            }

            send_ack(&mut ws, sequence_number, &mut stdout, message).await?;
        }
    }

    ws.close().await?;
    //stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    info!("Remote close");

    Ok(())
}

async fn send_ack(
    mut ws: &mut WebSocketStream<MaybeTlsStream<TcpStream>>,
    sequence_number: i64,
    stdout: &mut Stdout,
    message: ClientMessage,
) -> Result<()> {
    let ack = ssm::build_acknowledge(sequence_number, message.message_id);
    send_binary(&mut ws, ack, None).await?;
    debug!("Sent ack for message: {:?}", message.message_id);

    if message.payload_type == PayloadType::Output {
        stdout.write_all(message.payload.as_bytes()).await?;
        //stdout.execute(Print(&message.payload))?;
        //println!("{}", message.payload);
    } else {
        debug!("{:?}", message);
    }

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
) -> Result<()> {
    send_message(ws, Message::text(input), None).await?;

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
