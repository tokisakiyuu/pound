use anyhow::{Result, Ok};
use super::location::{Location, self};
use super::position::Position;
use super::range::Range;
use crate::editor::measure::char_width;
use crate::macros::log;
use super::character::Character;

#[derive(Default, Debug)]
pub struct Document {
  raw: Vec<char>,
  parsed: Vec<Character>
}

impl Document {
  pub fn insert(&mut self, location: Location, str: &str) -> Result<Vec<&Character>> {
    let len = str.chars().count();
    let start_index: usize;
    match self.get_character(location) {
      Some(pchar) => {
        let index = pchar.index;
        start_index = index;
        self.raw.splice(index..index, str.chars());
      },
      None => {
        start_index = self.raw.len();
        self.raw.extend(str.chars());
      },
    }
    self.parse();
    Ok(self.parsed.iter().skip(start_index).take(len).collect())
  }

  pub fn remove(&mut self, range: Range) -> Result<()> {
    let mut offset = 0;
    for pchar in self.parsed.iter() {
      let location = pchar.location;
      if location >= range.start && location < range.end && pchar.index < self.raw.len() {
        self.raw.remove(pchar.index - offset);
        offset += 1;
      }
    }
    self.parse();
    Ok(())
  }

  pub fn get_character(&self, location: Location) -> Option<&Character> {
    self.parsed.iter().find(|c| c.location == location)
  }

  fn parse(&mut self) {
    self.parsed.clear();
    let mut location = Location { ln: 0, col: 0 };
    let mut position = Position { x: 0, y: 0 };
    for (index, char) in self.raw.iter().enumerate() {
      let char = char.clone();
      let width = char_width(char);
      if char == '\n' {
        location.ln += 1;
        location.col = 0;
        position.x = 0;
        position.y += 1;
      }
      self.parsed.push(
        Character {
          char: char.clone(),
          index,
          location: location.clone(),
          position: position.clone(),
          width,
        }
      );
      location.col += 1;
      position.x += width;
    }
  }

  pub fn lines(&self) -> Vec<Vec<Character>> {
    let mut lines: Vec<Vec<Character>> = Vec::new();
    let mut current_ln: u32 = 0;
    let mut current_line: Vec<Character> = Vec::new();
    for char in self.parsed.iter() {
      if char.char == '\n' {
        lines.push(current_line.clone());
        current_ln += 1;
        current_line = Vec::new();
      }
      if char.width == 0 {
        continue;
      }
      if char.location.ln == current_ln {
        current_line.push(char.clone());
      }
      if char.location.ln > current_ln {
        lines.push(current_line.clone());
        current_ln = char.location.ln;
        current_line = vec![char.clone()];
      }
      if char.location.ln < current_ln {
        panic!("document model error.");
      }
    }
    if !current_line.is_empty() {
      lines.push(current_line);
    }
    lines
  }

  pub fn before(&self, location: Location) -> Option<Character> {
    match self.get_character(location) {
      Some(pchar) => {
        if pchar.index > 0 {
          self.parsed.get(pchar.index - 1).cloned()
        } else {
          None
        }
      },
      None => self.last_character(),
    }
  }

  pub fn after(&self, location: Location) -> Option<Character> {
    match self.get_character(location) {
      Some(pchar) => {
        if pchar.index + 1 < self.parsed.len() {
          self.parsed.get(pchar.index + 1).cloned()
        } else {
          None
        }
      },
      None => None
    }
  }

  pub fn last_character(&self) -> Option<Character> {
    self.parsed.last().cloned()
  }

  pub fn is_out_of_document(&self, location: Location) -> bool {
    match self.last_character() {
      Some(char) => location > char.location,
      None => true,
    }
  }
}

impl From<&str> for Document {
  fn from(value: &str) -> Self {
    let mut doc = Self {
      raw: value.chars().collect(),
      parsed: Vec::new()
    };
    doc.parse();
    doc
  }
}

impl Into<String> for Document {
  fn into(self) -> String {
    self.raw.into_iter().collect()
  }
}

#[cfg(test)]
mod tests {
use super::*;

  #[test]
  fn test1() {
    let mut doc = Document::from("Hello你好\nWorld世界");
    let pchar = doc.get_character(Location { ln: 1, col: 2 }).unwrap();
    assert_eq!(pchar.char, 'r');
    doc.remove(
      Range {
        start: Location { ln: 0, col: 7 },
        end: Location { ln: 0, col: 8 },
      }
    ).unwrap();
    doc.insert(Location { ln: 0, col: 7 }, "，").unwrap();
    let result: String = doc.into();
    assert_eq!(result, "Hello你好，World世界".to_string());
  }

  #[test]
  fn test2() {
    let mut doc = Document::default();
    doc.insert(Location { ln: 0, col: 0 }, "nihao你好").unwrap();
    doc.insert(Location { ln: 0, col: 99 }, "，世界").unwrap();
    doc.insert(Location { ln: 0, col: 8 }, "我的").unwrap();
    let start = Location { ln: 0, col: 0 };
    let end = Location { ln: 0, col: 5 };
    doc.remove(Range::new(start, end)).unwrap();
    assert_eq!(doc.raw.into_iter().collect::<String>(), "你好，我的世界");
  }

  #[test]
  fn test3() {
    let doc = Document::from("rust\n铁锈");
    let result = doc.lines()
      .iter()
      .map(|line| {
        line
          .iter()
          .map(|c| c.char)
          .collect::<String>()
      })
      .collect::<Vec<String>>()
      .join("\n");
    assert_eq!(result, "rust\n铁锈");
  }

  #[test]
  fn test4() {
    let doc = Document::from("rust\n铁锈");
    let before_char = doc.before(Location { ln: 1, col: 1 }).unwrap();
    assert_eq!(before_char.char, '\n');
    assert_eq!(before_char.location, Location { ln: 1, col: 0 });
    let after_char = doc.after(Location { ln: 1, col: 0 }).unwrap();
    assert_eq!(after_char.char, '铁');
    assert_eq!(after_char.location, Location { ln: 1, col: 1 });
  }

  #[test]
  fn test5() {
    let mut doc = Document::default();
    doc.insert(Location { ln: 0, col: 0 }, "\n").unwrap();
    doc.insert(Location { ln: 1, col: 1 }, "\n").unwrap();
    doc.insert(Location { ln: 2, col: 1 }, "1").unwrap();
    doc.insert(Location { ln: 2, col: 2 }, "2").unwrap();
    doc.insert(Location { ln: 2, col: 3 }, "3").unwrap();
    for line in doc.lines() {
      println!("{:?}", line.iter().map(|c| c.char).collect::<Vec<char>>());
    }
  }
}
