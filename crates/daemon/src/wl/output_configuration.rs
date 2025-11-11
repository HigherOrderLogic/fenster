use wayland_client::{Connection, Dispatch, Proxy, QueueHandle};
use wayland_protocols_wlr::output_management::v1::client::zwlr_output_configuration_v1::ZwlrOutputConfigurationV1;

use crate::wl::State;

impl Dispatch<ZwlrOutputConfigurationV1, ()> for State {
  fn event(
    _: &mut Self,
    _: &ZwlrOutputConfigurationV1,
    _: <ZwlrOutputConfigurationV1 as Proxy>::Event,
    _: &(),
    _: &Connection,
    _: &QueueHandle<Self>,
  ) {
  }
}
