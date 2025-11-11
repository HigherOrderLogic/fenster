use gpui::{IntoElement, Render, Styled, div};

pub struct Fenster;

impl Render for Fenster {
  fn render(&mut self, _: &mut gpui::Window, _: &mut gpui::Context<Self>) -> impl IntoElement {
    div().size_full()
  }
}
