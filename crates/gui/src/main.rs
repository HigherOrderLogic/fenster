mod fenster;

use std::env::args_os;

use anyhow::Error as AnyError;
use fenster_common::utils::get_log_level;
use gpui::{AppContext, Application, Bounds, WindowBounds, WindowOptions};
use tracing_subscriber::{EnvFilter, fmt as fmt_subscriber};

use crate::fenster::Fenster;

fn main() -> Result<(), AnyError> {
  fmt_subscriber()
    .with_env_filter(
      EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| format!("{}={}", env!("CARGO_BIN_NAME"), get_log_level(args_os().skip(1))).into()),
    )
    .init();

  Application::new().run(|cx| {
    let bounds = Bounds::maximized(None, cx);
    cx.open_window(
      WindowOptions {
        app_id: Some(String::from("io.kamn.Fenster")),
        window_bounds: Some(WindowBounds::Maximized(bounds)),
        ..Default::default()
      },
      |_, cx| {
        cx.activate(true);
        cx.new(|_| Fenster)
      },
    )
    .expect("failed to open windows");
  });

  Ok(())
}
