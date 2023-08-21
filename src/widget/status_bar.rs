use ratatui::{prelude::*, widgets::*};

#[derive(Default)]
pub struct StatusBar;

impl Widget for StatusBar {
  fn render(self, area: Rect, buf: &mut Buffer) {
    buf.set_style(area, Style::default().bg(Color::Blue));
    buf.set_string(area.left(), area.top(), "Status Bar", Style::default());
  }
}
