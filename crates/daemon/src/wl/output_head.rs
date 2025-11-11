use parking_lot::RwLock;
use wayland_client::{
  Connection, Dispatch, Proxy, QueueHandle, WEnum, event_created_child, protocol::wl_output::Transform,
};
use wayland_protocols_wlr::output_management::v1::client::{
  zwlr_output_head_v1::{AdaptiveSyncState, Event as ZwlrOutputHeadEvent, ZwlrOutputHeadV1},
  zwlr_output_manager_v1::ZwlrOutputManagerV1,
  zwlr_output_mode_v1::ZwlrOutputModeV1,
};

use crate::wl::{OutputTransform, ProxyTypedData, State};

impl Dispatch<ZwlrOutputHeadV1, ()> for State {
  fn event(
    state: &mut Self,
    proxy: &ZwlrOutputHeadV1,
    event: <ZwlrOutputHeadV1 as Proxy>::Event,
    _: &(),
    _: &Connection,
    _: &QueueHandle<Self>,
  ) {
    let Some(head) = state.output_heads.get_mut(&proxy.id()) else {
      tracing::debug!("unknown head {}", proxy.id());
      return;
    };

    match event {
      ZwlrOutputHeadEvent::Name { name } => head.name = Some(name),
      ZwlrOutputHeadEvent::Description { description } => head.description = Some(description),
      ZwlrOutputHeadEvent::PhysicalSize { width, height } => head.physical_size = Some((width, height)),

      ZwlrOutputHeadEvent::Mode { mode } => {
        *mode.typed_data().write() = Some(proxy.id());
        head.add_mode(mode);
      }
      ZwlrOutputHeadEvent::Enabled { enabled } => head.enabled = enabled != 0,
      ZwlrOutputHeadEvent::CurrentMode { mode } => head.current_mode = Some(mode.id()),
      ZwlrOutputHeadEvent::Position { x, y } => head.position = (x, y),
      ZwlrOutputHeadEvent::Transform { transform } => {
        if let WEnum::Value(t) = transform {
          let (deg, flipped) = match t {
            Transform::Normal => (0, false),
            Transform::_90 => (90, false),
            Transform::_180 => (180, false),
            Transform::_270 => (270, false),
            Transform::Flipped => (0, true),
            Transform::Flipped90 => (90, true),
            Transform::Flipped180 => (180, true),
            Transform::Flipped270 => (270, true),
            u => {
              tracing::debug!("unknown output transform: {:?}", u);
              return;
            }
          };
          head.transform = OutputTransform { deg, flipped };
        }
      }
      ZwlrOutputHeadEvent::Scale { scale } => head.scale = scale,

      ZwlrOutputHeadEvent::Make { make } => head.manufacturer = Some(make),
      ZwlrOutputHeadEvent::Model { model } => head.model = Some(model),
      ZwlrOutputHeadEvent::SerialNumber { serial_number } => head.serial_number = Some(serial_number),
      ZwlrOutputHeadEvent::AdaptiveSync { state } => {
        if let WEnum::Value(s) = state {
          head.adadptive_sync = matches!(s, AdaptiveSyncState::Enabled);
        }
      }
      ZwlrOutputHeadEvent::Finished => {
        proxy.release();
        state.output_heads.remove(&proxy.id());
      }
      e => tracing::debug!("unknown event: {:?}", e),
    }
  }

  event_created_child!(State, ZwlrOutputManagerV1, [EVT_MODE_OPCODE => (ZwlrOutputModeV1, RwLock::new(None))]);
}
