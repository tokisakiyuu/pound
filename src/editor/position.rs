#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Position {
  pub x: u32,
  pub y: u32,
}

impl Position {
  pub fn top(&self) -> Self {
    Self { x: self.x, y: self.y.saturating_sub(1) }
  }

  pub fn right(&self) -> Self {
    Self { x: self.x.saturating_add(1), y: self.y }
  }

  pub fn bottom(&self) -> Self {
    Self { x: self.x, y: self.y.saturating_add(1) }
  }

  pub fn left(&self) -> Self {
    Self { x: self.x.saturating_sub(1), y: self.y }
  }

  pub fn new(x: u32, y: u32) -> Self {
    Self { x, y }
  }
}
