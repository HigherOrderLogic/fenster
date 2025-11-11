use wayland_client::{Connection, Dispatch, Proxy, QueueHandle};
use wayland_protocols_wlr::output_management::v1::client::zwlr_output_manager_v1::{
  Event as ZwlrOutputManagerEvent, ZwlrOutputManagerV1,
};

use super::State;

impl Dispatch<ZwlrOutputManagerV1, ()> for State {
  fn event(
    state: &mut Self,
    _: &ZwlrOutputManagerV1,
    event: <ZwlrOutputManagerV1 as Proxy>::Event,
    _: &(),
    _: &Connection,
    _: &QueueHandle<Self>,
  ) {
    match event {
      ZwlrOutputManagerEvent::Head { head } => state.add_head(head),
      ZwlrOutputManagerEvent::Done { serial } => state.output_manager_serial = serial,
      ZwlrOutputManagerEvent::Finished => {
        state.output_manager = None;
        state.output_manager_serial = 0;
      }
      e => tracing::debug!("unknown event: {:?}", e),
    }
  }
}
