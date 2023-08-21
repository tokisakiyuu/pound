use ratatui::{prelude::*, widgets::*};

use crate::log;

#[derive(Default)]
pub struct Terminal;

impl Widget for Terminal {
  fn render(self, area: Rect, buf: &mut Buffer) {
    let lines = log::lines();
    let len = lines.len();
    let offset = 0;
    let display_lines: Vec<String> = if len - offset > area.height as usize {
      lines.into_iter()
        .skip(len - offset as usize - area.height as usize)
        .take(area.height as usize)
        .collect()
    } else {
      lines
    };
    for (y, line) in display_lines.into_iter().enumerate() {
      buf.set_string(area.left(), area.top() + y as u16, line.clone(), Style::default());
    }
  }
}
