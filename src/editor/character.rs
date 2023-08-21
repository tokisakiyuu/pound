use super::position::Position;
use super::location::Location;

#[derive(Clone, Debug, PartialEq)]
pub struct Character {
  pub index: usize,
  pub char: char,
  pub width: u32,
  pub position: Position,
  pub location: Location,
}
