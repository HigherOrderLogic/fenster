use wayland_client::{Connection, Dispatch, Proxy, QueueHandle};
use wayland_protocols_wlr::output_management::v1::client::zwlr_output_configuration_head_v1::ZwlrOutputConfigurationHeadV1;

use crate::wl::State;

impl Dispatch<ZwlrOutputConfigurationHeadV1, ()> for State {
  fn event(
    _: &mut Self,
    _: &ZwlrOutputConfigurationHeadV1,
    _: <ZwlrOutputConfigurationHeadV1 as Proxy>::Event,
    _: &(),
    _: &Connection,
    _: &QueueHandle<Self>,
  ) {
  }
}
