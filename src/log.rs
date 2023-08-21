use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
  static ref LOG: Mutex<Vec<String>> = Mutex::new(Vec::new());
  static ref OFFSET: u32 = 0;
}

pub fn info(str: &str) {
  let mut lines = LOG.lock().unwrap();
  let new_lines: Vec<&str> = str.split('\n').collect::<Vec<&str>>();
  for line in new_lines {
    lines.push(String::from(line));
  }
}

pub fn lines() -> Vec<String> {
  LOG.lock().unwrap().clone()
}
