use crate::enums::{EMessageType, EPayloadType};
use crate::structs::{AgentMessage, Token};
use aws_sdk_ssm::types::InstanceInformationStringFilter;
use bytes::Bytes;
use crossterm::event::KeyCode;
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{cursor, terminal, ExecutableCommand};
use futures_util::{SinkExt, StreamExt};
use std::io::{self, stdout, Write};
use tokio_websockets::Message;
use tracing::level_filters::LevelFilter;
use tracing::{debug, info};
use tracing_subscriber::EnvFilter;

mod enums;
mod helpers;
mod ssm;
mod structs;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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

    let url = session.stream_url.unwrap();
    debug!("Session URL: {}", url);

    let (mut ws, _) = tokio_websockets::ClientBuilder::new()
        .uri(&url)
        .unwrap()
        .connect()
        .await?;

    debug!("{:?}", ws);

    let mut sequence_number = 0i64;

    let token =
        Token::build_token_message(session.session_id.unwrap(), session.token_value.unwrap());
    let token_json = serde_json::to_string_pretty(&token).unwrap();
    debug!("Token: {}", token_json);
    ws.send(Message::text(token_json)).await?;

    let terminal_size = terminal::size()?;

    let term_options = structs::TermOptions {
        cols: terminal_size.0,
        rows: terminal_size.1,
    };
    let init_message = ssm::build_init_message(term_options, sequence_number);
    ws.send(Message::binary(Bytes::from(init_message))).await?;
    sequence_number += 1;

    loop {
        match crossterm::event::read()? {
            crossterm::event::Event::Key(key_event) => match key_event.code {
                KeyCode::Backspace => {}
                KeyCode::Enter => {}
                KeyCode::Left => {}
                KeyCode::Right => {}
                KeyCode::Up => {}
                KeyCode::Down => {}
                KeyCode::Home => {}
                KeyCode::End => {}
                KeyCode::PageUp => {}
                KeyCode::PageDown => {}
                KeyCode::Tab => {}
                KeyCode::BackTab => {}
                KeyCode::Delete => {}
                KeyCode::Insert => {}
                KeyCode::F(_) => {}
                KeyCode::Char(c) => {
                    let input = ssm::build_input_message(&c.to_string(), sequence_number);
                    ws.send(Message::binary(Bytes::from(input))).await?;
                    sequence_number += 1;
                }
                KeyCode::Null => {}
                KeyCode::Esc => break,
                KeyCode::CapsLock => {}
                KeyCode::ScrollLock => {}
                KeyCode::NumLock => {}
                KeyCode::PrintScreen => {}
                KeyCode::Pause => {}
                KeyCode::Menu => {}
                KeyCode::KeypadBegin => {}
                KeyCode::Media(_) => {}
                KeyCode::Modifier(_) => {}
            },
            _ => {}
        }

        if let Some(Ok(msg)) = ws.next().await {
            if msg.is_close() {
                break;
            }

            let bytes = msg.as_payload().iter().as_slice();
            let message = AgentMessage::bytes_to_message(bytes);

            if message.message_type != EMessageType::Acknowledge {
                let ack = ssm::build_acknowledge(sequence_number, &message.message_id);
                ws.send(Message::binary(Bytes::from(ack))).await?;
                debug!("Sent ack for message: {:?}", message.message_id);
                sequence_number += 1;
            }

            if message.payload_type == EPayloadType::Output {
                //stdout.execute(Clear(ClearType::All))?;
                stdout.execute(Print(message.payload))?;
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
