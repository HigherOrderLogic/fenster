use wayland_client::{DispatchError, EventQueue};

use crate::wl::State;

pub struct App {
  pub state: State,
  pub event_queue: EventQueue<State>,
}

impl App {
  pub async fn dispatch(&mut self) -> Result<usize, DispatchError> {
    self.state.dispatch(&mut self.event_queue).await
  }
}
