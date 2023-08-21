use crossterm::event::{Event, KeyCode};
use crate::{state::AppState, macros::log};

pub fn event_handler(event: Event, state: &mut AppState) {
  if let Event::Key(key) = event {
    log!("{:?}", key);
    if let KeyCode::Char(char) = key.code {
      state.editor.input(&format!("{}", char));
    }
    if let KeyCode::Backspace = key.code {
      state.editor.backspace();
    }
    if let KeyCode::Enter = key.code {
      state.editor.input("\n");
    }
  }
}
