use parking_lot::RwLock;
use wayland_client::{Connection, Dispatch, Proxy, QueueHandle, backend::ObjectId};
use wayland_protocols_wlr::output_management::v1::client::zwlr_output_mode_v1::{
  Event as ZwlrOutputModeEvent, ZwlrOutputModeV1,
};

use crate::wl::State;

impl Dispatch<ZwlrOutputModeV1, RwLock<Option<ObjectId>>> for State {
  fn event(
    state: &mut Self,
    proxy: &ZwlrOutputModeV1,
    event: <ZwlrOutputModeV1 as Proxy>::Event,
    data: &RwLock<Option<ObjectId>>,
    _: &Connection,
    _: &QueueHandle<Self>,
  ) {
    let Some(head_id) = data.read().clone() else {
      return;
    };
    let Some(head) = state.output_heads.get_mut(&head_id) else {
      return;
    };
    let mode = head.modes.entry(proxy.id()).or_insert_with(|| proxy.clone().into());

    match event {
      ZwlrOutputModeEvent::Size { width, height } => mode.size = (width, height),
      ZwlrOutputModeEvent::Refresh { refresh } => mode.refresh = refresh,
      ZwlrOutputModeEvent::Preferred => mode.preferred = true,
      ZwlrOutputModeEvent::Finished => {
        proxy.release();
        head.modes.remove(&proxy.id());
      }
      _ => todo!(),
    }
  }
}
