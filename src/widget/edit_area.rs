use ratatui::{prelude::*, widgets::*};

use crate::state::AppState;

#[derive(Default)]
pub struct EditArea;

impl StatefulWidget for EditArea {
  type State = AppState;
  fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
    state.editor.set_size(area.width as u32, area.height as u32);
    let visual_area = state.editor.visual_area();
    for (i, line) in visual_area.iter().enumerate() {
      buf.set_string(area.left(), area.top() + i as u16, line, Style::default());
    }
  }
}
