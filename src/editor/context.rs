use super::{location::Location, document::Document, range::Range, position::Position};
use crate::macros::log;

pub struct Context {
  viewport_size: ViewportSize,
  cursor_location: Location,
  offset_top: u32,
  offset_left: u32,
  document: Document,
}

impl Context {
  pub fn with_size(width: u32, height: u32) -> Self {
    Self {
      viewport_size: ViewportSize { width, height },
      document: Document::default(),
      cursor_location: Location { ln: 0, col: 0 },
      offset_left: 0,
      offset_top: 0,
    }
  }

  pub fn input(&mut self, content: &str) {
    if let Ok(chars) = self.document.insert(self.cursor_location, content) {
      if let Some(end_char) = chars.last() {
        let new_cursor_location = {
          let mut loc = end_char.location.clone();
          loc.col += 1;
          loc
        };
        self.cursor_location = new_cursor_location;
        self.auto_center_cursor();
      }
    }
  }

  pub fn backspace(&mut self) {
    if let Some(char) = self.document.before(self.cursor_location) {
      if self.document.remove(Range::new(char.location, self.cursor_location)).is_ok() {
        self.cursor_location = char.location;
        self.auto_center_cursor();
      }
    }
  }

  pub fn visual_area(&self) -> Vec<String> {
    let width = self.viewport_size.width;
    let offset_left = self.offset_left;
    let cuted_lines: Vec<String> = self.document.lines().iter()
      .skip(self.offset_top as usize)
      .take(self.viewport_size.height as usize)
      .map(|line| {
        let mut str = String::new();
        let mut cut_used: u32 = 0;
        let mut lenght_used: u32 = 0;
        for char in line {
          if cut_used < offset_left {
            // stage 1
            if cut_used + char.width <= offset_left {
              cut_used += char.width;
            } else {
              let slot: String = std::iter::repeat("<").take((cut_used + char.width - offset_left) as usize).collect();
              str = format!("{}{}", str, slot);
              cut_used = offset_left;
            }
          } else {
            // stage 2
            if lenght_used < width {
              // stage 3
              if lenght_used + char.width <= width {
                lenght_used += char.width;
                str.push(char.char);
              } else {
                let slot: String = std::iter::repeat(">").take((char.width - (lenght_used + char.width - width)) as usize).collect();
                str = format!("{}{}", str, slot);
                lenght_used = width;
              }
            } else {
              // stage 4
              // noop
            }
          }
        }
        str
      })
      .collect();
    cuted_lines
  }

  pub fn delete(&mut self) { todo!() }
  pub fn up(&mut self) { todo!() }
  pub fn down(&mut self) { todo!() }
  pub fn left(&mut self) { todo!() }
  pub fn right(&mut self) { todo!() }
  pub fn set_cursor(&mut self, location: Location) { todo!() }
  pub fn select(&mut self, start: Location, end: Location) { todo!() }
  
  pub fn scroll_up(&mut self) {
    self.offset_top += 1;
  }

  pub fn scroll_down(&mut self) {
    self.offset_top = self.offset_top.saturating_sub(1);
  }

  pub fn scroll_left(&mut self) {
    self.offset_left += 1;
  }

  pub fn scroll_right(&mut self) {
    self.offset_left = self.offset_left.saturating_sub(1);
  }

  pub fn set_size(&mut self, width: u32, height: u32) {
    self.viewport_size = ViewportSize { width, height };
    self.auto_center_cursor();
  }

  pub fn cursor_position(&self) -> Position {
    let mut pos = self.absolute_cursor_position();
    pos.x = pos.x.saturating_sub(self.offset_left);
    pos.y = pos.y.saturating_sub(self.offset_top);
    pos
  }

  fn absolute_cursor_position(&self) -> Position {
    match self.document.get_character(self.cursor_location) {
      Some(c) => c.position.clone(),
      None => {
        match self.document.last_character() {
          Some(lc) => {
            let mut pos = lc.position.clone();
            pos.x += lc.width;
            pos
          },
          None => Position { x: 0, y: 0 },
        }
      },
    }
  }

  fn auto_center_cursor(&mut self) {
    let cursor_pos = self.cursor_position();
    let abs_pos = self.absolute_cursor_position();
    if cursor_pos.x > self.viewport_size.width {
      self.offset_left += self.viewport_size.width / 2;
    }
    if cursor_pos.y > self.viewport_size.height - 1 {
      self.offset_top += 1;
    }
    if abs_pos.x < self.offset_left {
      self.offset_left = self.offset_left.saturating_sub(self.viewport_size.width / 2);
    }
    if abs_pos.y < self.offset_top {
      self.offset_top = self.offset_top.saturating_sub(1);
    }
  }

}

struct ViewportSize {
  width: u32,
  height: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
      let mut editor = Context::with_size(9, 9);
      for ln in 0..9 {
        for col in 0..9 {
          if ln == col {
            editor.input("汉");
          } else {
            editor.input(&format!("{}", ln));
          }
        }
        editor.input("\n");
      }
      for _ in 0..3 {
        editor.scroll_up();
      }
      for _ in 0..4 {
        editor.scroll_left();
      }
      let s = &editor.visual_area()[0];
      assert_eq!(s, "<33333");
    }
}
