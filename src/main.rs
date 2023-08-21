use std::{
  io::{self, Stdout},
  time::Duration,
};

use anyhow::{Context, Result};
use crossterm::{
  event::{self, Event, KeyCode, KeyModifiers, EnableMouseCapture, DisableMouseCapture},
  execute,
  terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use macros::log;
use ratatui::prelude::*;

mod view;
mod state;
mod handler;
mod macros;
mod utils;
mod editor;
mod widget;
mod log;

type Frame<'a> = ratatui::Frame<'a, CrosstermBackend<Stdout>>;

fn main() -> Result<()> {
  setup_terminal().context("setup failed")?;
  let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout())).expect("creating terminal failed");
  run(&mut terminal).context("app loop paniced")?;
  restore_terminal().context("restore terminal failed")?;
  Ok(())
}

/// Setup the terminal. Enable raw mode, enter the alternate screen, and hide the cursor.
fn setup_terminal() -> Result<()> {
  enable_raw_mode().context("failed to enable raw mode")?;
  execute!(
    io::stdout(),
    crossterm::cursor::Hide,
    EnterAlternateScreen,
    EnableMouseCapture,
  ).context("unable to enter alternate screen")?;
  let default_hook = std::panic::take_hook();
  std::panic::set_hook(Box::new(move |info| {
    restore_terminal().expect("restore terminal failed");
    default_hook(info);
  }));
  Ok(())
}

/// Restore the terminal. Disable raw mode, leave the alternate screen, and show the cursor.
fn restore_terminal() -> Result<()> {
  disable_raw_mode().context("failed to disable raw mode")?;
  execute!(
    io::stdout(),
    DisableMouseCapture,
    LeaveAlternateScreen,
    crossterm::cursor::Show,
  ).context("unable to switch to main screen")?;
  Ok(())
}

/// Run the application loop. This is where is handle events and update the application state.
fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
  let mut app_state = state::AppState::default();
  loop {
    terminal.draw(|frame| render_app(frame, &mut app_state))?;
    if event_poll(&mut app_state)? { break }
  }
  Ok(())
}

/// Render the application. This is where is draw the application UI.
fn render_app(frame: &mut Frame, state: &mut state::AppState) {
  let (top_window, status_bar, bottom_window) = view::create_areas(frame, state);
  frame.render_stateful_widget(widget::EditArea, top_window, state);
  frame.render_widget(widget::StatusBar, status_bar);
  frame.render_widget(widget::Terminal, bottom_window);
  let cursor_pos = state.editor.cursor_position();
  frame.set_cursor(top_window.left() + cursor_pos.x as u16, top_window.top() + cursor_pos.y as u16);
}

fn event_poll(state: &mut state::AppState) -> Result<bool> {
  if event::poll(Duration::from_millis(250)).context("event poll failed")? {
    let event = event::read().context("event read failed")?;
    if let Event::Key(key) =  event {
      if matches!(key.code, KeyCode::Char('c')) && matches!(key.modifiers, KeyModifiers::CONTROL) {
        return Ok(true)
      }
    }
    handler::event_handler(event, state);
  }
  Ok(false)
}
