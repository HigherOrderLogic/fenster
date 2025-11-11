mod app;
mod listener;
mod wl;

use std::{
  env::args_os, io::ErrorKind as IoErrorKind, os::unix::net::UnixListener, sync::Arc, thread::available_parallelism,
};

use anyhow::{Context, Error as AnyError};
use async_fs::create_dir_all;
use async_global_executor::{GlobalExecutorConfig, block_on, init_with_config};
use async_io::Async;
use async_lock::Mutex;
use fenster_common::{paths::get_socket_path, utils::get_log_level};
use tracing_subscriber::{EnvFilter, fmt as fmt_subscriber};
use wayland_client::Connection;

use crate::{app::App, listener::start_listener, wl::State};

fn main() -> Result<(), AnyError> {
  fmt_subscriber()
    .with_env_filter(
      EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| format!("{}={}", env!("CARGO_BIN_NAME"), get_log_level(args_os().skip(1))).into()),
    )
    .init();

  init_with_config(GlobalExecutorConfig::default().with_max_threads(available_parallelism().map_or(1, |n| n.get())));
  block_on(async_main())
}

async fn async_main() -> Result<(), AnyError> {
  let conn = Connection::connect_to_env().context("failed to connect to wayland server")?;
  let mut event_queue = conn.new_event_queue();
  let handle = event_queue.handle();
  let wl_registry = conn.display().get_registry(&handle, ());

  let mut state = State {
    conn,
    handle,
    output_heads: Default::default(),
    wl_registry,
    output_manager: Default::default(),
    output_manager_serial: Default::default(),
  };

  if let Err(e) = event_queue.roundtrip(&mut state) {
    tracing::warn!("failed to roundtrip: {}", e);
  }

  let app = Mutex::new(App { state, event_queue }).into();

  let socket_path = get_socket_path();
  if let Some(socket_dir) = socket_path.parent()
    && !socket_dir.exists()
  {
    create_dir_all(socket_dir)
      .await
      .context("failed to create socket path")?;
  }

  loop {
    let listener = match Async::<UnixListener>::bind(socket_path.to_path_buf()) {
      Ok(l) => l,
      Err(e) => {
        if matches!(e.kind(), IoErrorKind::AlreadyExists) {
          tracing::error!("socket already exist at {}", socket_path.display());
          break;
        } else {
          tracing::error!("failed to create socket: {}", e);
          continue;
        }
      }
    };
    if let Err(e) = start_listener(listener, &app).await {
      tracing::error!("error while listening to client: {}", e)
    }
  }

  Ok(())
}
