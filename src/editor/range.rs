use super::location::Location;

#[derive(Clone, Copy, PartialEq)]
pub struct Range {
  pub start: Location,
  pub end: Location,
}

impl Range {
  pub fn new(start: Location, end: Location) -> Self {
    Self { start, end }
  }
}
