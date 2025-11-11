mod output_configuration;
mod output_configuration_head;
mod output_head;
mod output_manager;
mod output_mode;
mod wl_registry;

use std::{collections::HashMap, io::ErrorKind as IoErrorKind};

use async_io::Async;
use wayland_client::{
  Connection, Dispatch, DispatchError, EventQueue, Proxy, QueueHandle,
  backend::{ObjectId, WaylandError},
  protocol::wl_registry::WlRegistry,
};
use wayland_protocols_wlr::output_management::v1::client::{
  zwlr_output_head_v1::ZwlrOutputHeadV1, zwlr_output_manager_v1::ZwlrOutputManagerV1,
  zwlr_output_mode_v1::ZwlrOutputModeV1,
};

pub struct OutputMode {
  pub wlr_mode: ZwlrOutputModeV1,
  pub size: (i32, i32),
  pub refresh: i32,
  pub preferred: bool,
}

impl From<ZwlrOutputModeV1> for OutputMode {
  fn from(wlr_mode: ZwlrOutputModeV1) -> Self {
    Self {
      wlr_mode,
      size: (0, 0),
      refresh: 0,
      preferred: false,
    }
  }
}

pub struct OutputTransform {
  pub deg: u32,
  pub flipped: bool,
}

pub struct OutputHead {
  pub wlr_head: ZwlrOutputHeadV1,
  pub name: Option<String>,
  pub description: Option<String>,
  pub physical_size: Option<(i32, i32)>,
  pub enabled: bool,
  pub modes: HashMap<ObjectId, OutputMode>,
  pub current_mode: Option<ObjectId>,
  pub position: (i32, i32),
  pub transform: OutputTransform,
  pub scale: f64,
  pub manufacturer: Option<String>,
  pub model: Option<String>,
  pub serial_number: Option<String>,
  pub adadptive_sync: bool,
}

impl From<ZwlrOutputHeadV1> for OutputHead {
  fn from(wlr_head: ZwlrOutputHeadV1) -> Self {
    Self {
      wlr_head,
      name: None,
      description: None,
      physical_size: None,
      enabled: false,
      modes: HashMap::new(),
      current_mode: None,
      position: (0, 0),
      transform: OutputTransform { deg: 0, flipped: false },
      scale: 1.,
      manufacturer: None,
      model: None,
      serial_number: None,
      adadptive_sync: false,
    }
  }
}

impl OutputHead {
  fn add_mode(&mut self, output_mode: ZwlrOutputModeV1) {
    self.modes.insert(output_mode.id(), output_mode.into());
  }
}

pub struct State {
  pub conn: Connection,
  pub handle: QueueHandle<Self>,
  pub output_heads: HashMap<ObjectId, OutputHead>,
  pub wl_registry: WlRegistry,
  pub output_manager: Option<ZwlrOutputManagerV1>,
  pub output_manager_serial: u32,
}

impl State {
  pub async fn dispatch(&mut self, event_queue: &mut EventQueue<Self>) -> Result<usize, DispatchError> {
    let dispatched = event_queue.dispatch_pending(self)?;
    if dispatched > 0 {
      return Ok(dispatched);
    }

    let conn = self.conn.clone();

    conn.flush()?;

    if let Some(guard) = conn.prepare_read() {
      let fd = Async::new(guard.connection_fd()).map_err(|e| DispatchError::Backend(WaylandError::Io(e)))?;
      loop {
        if let Err(e) = fd.readable().await {
          if matches!(e.kind(), IoErrorKind::Interrupted) {
            continue;
          } else {
            return Err(DispatchError::Backend(WaylandError::Io(e)));
          }
        }
        drop(fd);
        break;
      }

      if let Err(wle) = guard.read() {
        let WaylandError::Io(ref e) = wle else {
          return Err(DispatchError::Backend(wle));
        };

        if !matches!(e.kind(), IoErrorKind::WouldBlock) {
          return Err(DispatchError::Backend(wle));
        }
      }
    }

    event_queue.dispatch_pending(self)
  }

  fn add_head(&mut self, output_head: ZwlrOutputHeadV1) {
    self.output_heads.insert(output_head.id(), output_head.into());
  }
}

trait ProxyTypedData<T> {
  fn typed_data(&self) -> &T;
}

impl<T, D> ProxyTypedData<D> for T
where
  State: Dispatch<T, D>,
  T: Proxy,
  D: Send + Sync + 'static,
{
  fn typed_data(&self) -> &D {
    self.data().unwrap()
  }
}
