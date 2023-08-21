use crate::editor::Context;

pub struct AppState {
  pub editor: Context,
  pub show_log: bool,
}

impl Default for AppState {
  fn default() -> Self {
    Self {
      editor: Context::with_size(50, 50),
      show_log: true
    }
  }
}
