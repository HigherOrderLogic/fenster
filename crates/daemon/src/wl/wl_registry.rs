use wayland_client::{
  Connection, Dispatch, Proxy, QueueHandle,
  protocol::wl_registry::{Event as WlRegistryEvent, WlRegistry},
};
use wayland_protocols_wlr::output_management::v1::client::zwlr_output_manager_v1::ZwlrOutputManagerV1;

use super::State;

impl Dispatch<WlRegistry, ()> for State {
  fn event(
    state: &mut Self,
    registry: &WlRegistry,
    event: <WlRegistry as Proxy>::Event,
    _: &(),
    _: &Connection,
    qh: &QueueHandle<Self>,
  ) {
    if let WlRegistryEvent::Global {
      name,
      interface,
      version,
    } = event
      && &interface[..] == "zwlr_output_manager_v1"
    {
      state.output_manager = Some(registry.bind::<ZwlrOutputManagerV1, _, _>(name, version.min(4), qh, ()));
    }
  }
}
