//! Reference from [github:urwid/urwid](https://github.com/urwid/urwid/blob/master/urwid/old_str_util.py)

/// get a value what is how much need columns the incoming char by calculation.
/// **in this application**, we need ignore length of "\n" and "\r", so add check unicode 10 and 13.
pub fn char_width(char: char) -> u32 {
  let value = char as u32;
  if value == 14 || value == 15 || value == 10 || value == 13 {
    return 0
  }
  for (num, width) in WIDTHS {
    if (char as u32) <= num {
      return width
    }
  }
  1
}

/// it's a tuple that combination of unicode and the columns number needed.
/// e.g. (159, 0) to (687, 1) means all char between unicode 159-687 need 
/// 1 unit of length.
const WIDTHS: [(u32, u32); 39] = [
  (126, 1),
  (159, 0),
  (687, 1),
  (710, 0),
  (711, 1),
  (727, 0),
  (733, 1),
  (879, 0),
  (1154, 1),
  (1161, 0),
  (4347, 1),
  (4447, 2),
  (7467, 1),
  (7521, 0),
  (8369, 1),
  (8426, 0),
  (9000, 1),
  (9002, 2),
  (11021, 1),
  (12350, 2),
  (12351, 1),
  (12438, 2),
  (12442, 0),
  (19893, 2),
  (19967, 1),
  (55203, 2),
  (63743, 1),
  (64106, 2),
  (65039, 1),
  (65059, 0),
  (65131, 2),
  (65279, 1),
  (65376, 2),
  (65500, 1),
  (65510, 2),
  (120831, 1),
  (130047, 1),
  (262141, 2),
  (1114109, 1),
];
