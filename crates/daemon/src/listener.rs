use std::{
  io::ErrorKind as IoErrorKind,
  os::unix::net::{UnixListener, UnixStream},
  sync::Arc,
};

use anyhow::{Context, Error as AnyError};
use async_global_executor::spawn;
use async_io::Async;
use async_lock::Mutex;
use fenster_ipc::{Message, Response, ResponseContent, ResponseError};
use futures_util::{
  AsyncBufReadExt, AsyncReadExt, AsyncWriteExt,
  io::{BufReader, BufWriter},
};
use serde_json::{error::Category, from_slice, to_string};
use tracing::{Instrument, Level, span};

use crate::app::App;

pub async fn start_listener(listener: Async<UnixListener>, app: &Arc<Mutex<App>>) -> Result<(), AnyError> {
  loop {
    match listener.accept().await {
      Ok((stream, addr)) => {
        let client_span = span!(Level::DEBUG, "client", addr = format!("{:?}", addr));
        let app_clone = app.clone();
        spawn(
          async {
            tracing::info!("client connected");
            if let Err(e) = handle_client(stream, app_clone).await {
              tracing::error!("error while handling client: {}", e);
            }
            tracing::info!("client disconnected");
          }
          .instrument(client_span),
        )
        .detach();
      }
      Err(e) => {
        if !matches!(e.kind(), IoErrorKind::WouldBlock) {
          return Err(e.into());
        }
      }
    }
  }
}

async fn handle_client(stream: Async<UnixStream>, app: Arc<Mutex<App>>) -> Result<(), AnyError> {
  let (read, write) = stream.split();
  let mut reader = BufReader::new(read);
  let mut writer = BufWriter::new(write);
  loop {
    let mut buf = Vec::new();
    match reader.read_until(b'\n', &mut buf).await {
      Ok(l) => {
        if l == 0 {
          tracing::debug!("empty message received, disconnecting");
          break;
        }
      }
      Err(e) => {
        if matches!(e.kind(), IoErrorKind::BrokenPipe) {
          break;
        } else {
          return Err(e.into());
        }
      }
    }
    let resp = match from_slice(&buf) {
      Ok(m) => match m {
        Message::Foo => {
          let mut app = app.lock().await;
          app.dispatch().await?;
          Response::success(ResponseContent::Foo)
        }
      },
      Err(e) => {
        let error_msg = match e.classify() {
          Category::Io => "failed to read/write IO stream",
          Category::Syntax => "messages contains invalid JSON",
          Category::Data => "message JSON contain incorrect type data",
          Category::Eof => "unexpected end of file",
        };
        Response::error(ResponseError::MalformedMessage(error_msg.to_owned()))
      }
    };
    let mut buf = to_string(&resp).unwrap();
    buf.push('\n');
    writer
      .write_all(&buf.into_bytes())
      .await
      .context("error writing response")?;
  }
  Ok(())
}
