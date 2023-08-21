#[allow(unused_macros)]
macro_rules! beep {
  () => {
    print!("\x07");
  }
}

#[allow(unused_imports)]
pub(crate) use beep;

macro_rules! log {
  ($($arg:tt)*) => {{
    crate::log::info(&format!($($arg)*));
  }};
}

pub(crate) use log;
