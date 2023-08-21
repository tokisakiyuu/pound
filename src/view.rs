use std::io::Stdout;
use ratatui::{
  prelude::*,
  widgets::*,
};

use crate::{state::AppState, editor::document::Document};

type Frame<'a> = ratatui::Frame<'a, CrosstermBackend<Stdout>>;

pub fn create_areas(frame: &mut Frame, state: &mut AppState) -> (Rect, Rect, Rect) {
  let area = frame.size();
  let whole = Layout::new()
    .direction(Direction::Vertical)
    .constraints([
      Constraint::Min(1),
      Constraint::Length(1),
      Constraint::Percentage(if state.show_log { 20 } else { 0 }),
    ])
    .split(area)
    .to_vec();
  (whole[0], whole[1], whole[2])
}

/// render line numbers like the left side of vscode edit area.
pub fn render_line_numbers(frame: &mut Frame, area: Rect, state: &mut AppState) {
  let line_number_layout = Layout::new()
    .direction(Direction::Vertical)
    .constraints(vec![Constraint::Length(1); area.height as usize])
    .split(area);
  // for (index, line) in line_number_layout.iter().enumerate() {
  //   let mut num = Paragraph::new((index + 1).to_string())
  //     .alignment(Alignment::Right)
  //     .dark_gray();
  //   if index == state.editor.cursor_location().ln as usize {
  //     num = num.white();
  //   }
  //   frame.render_widget(num, line.clone());
  // }
}

pub fn render_statusbar(frame: &mut Frame, area: Rect, state: &mut AppState) {
  let statusbar = Block::new()
    .bg(Color::Blue)
    .borders(Borders::NONE);
  let inner_area = statusbar.inner(area);
  let text = Paragraph::new("Editor")
    .alignment(Alignment::Right)
    .bold();
  frame.render_widget(statusbar, area);
  frame.render_widget(text, inner_area);
}

pub fn render_edit_area(frame: &mut Frame, area: Rect, state: &mut AppState) {
  // let block = Block::new().padding(Padding { left: 1, right: 0, top: 0, bottom: 0 });
  // let content_rect = block.inner(area);
  // let line_count = content_rect.height;
  // state.editor.set_size(content_rect.width as u32, content_rect.height as u32);
  // let layout = Layout::new()
  //   .direction(Direction::Vertical)
  //   .constraints(vec![Constraint::Length(1); line_count as usize])
  //   .split(content_rect);
  // // render lines
  // let lines = lines_walk(state.editor.document());
  // for (index, line) in lines.into_iter().enumerate() {
  //   render_edit_line(frame, layout[index as usize], line);
  // }
  // // render cursor
  // let cursor_location = state.editor.cursor_location();
  // if let Some(coords) = state.editor.document().get_char_coordinate(cursor_location.clone()) {
  //   if let Some(coord) = coords.first() {
  //     frame.set_cursor(content_rect.x + coord.0 as u16, coord.1 as u16);
  //   }
  // }
}

pub fn render_edit_line(frame: &mut Frame, area: Rect, content: String) {
  let p = Paragraph::new(content);
  frame.render_widget(p, area);
}

pub fn lines_walk(doc: &Document) -> Vec<String> {
  let mut current_line = 0;
  let mut line_str = String::new();
  let mut lines: Vec<String> = Vec::new();
  // for font in doc.fonts() {
  //   if font.location().ln == current_line {
  //     line_str.push(*font.char())
  //   } else {
  //     lines.push(line_str.clone());
  //     line_str.clear();
  //     current_line += 1;
  //   }
  // }
  lines
}

#[cfg(test)]
mod tests {
  #[test]
  fn test() {
    let line_len = 10;
    let content = String::from("0123456789");
    let c_len = content.len();
    println!("{} {}", c_len / line_len, c_len % line_len);
  }

  #[test]
  fn test2() {
    println!("{}", u16::MAX);
  }
}
