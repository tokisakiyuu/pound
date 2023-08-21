use std::cmp::Ordering;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Location {
  pub ln: u32,
  pub col: u32,
}

impl PartialOrd for Location {
  fn lt(&self, other: &Self) -> bool {
    let Location { ln, col } = self;
    let Location { ln: oln, col: ocol } = other;
    ln < oln || (ln == oln && col < ocol)
  }

  fn gt(&self, other: &Self) -> bool {
    let Location { ln, col } = self;
    let Location { ln: oln, col: ocol } = other;
    ln > oln || (ln == oln && col > ocol)
  }

  fn le(&self, other: &Self) -> bool {
    self.lt(other) || self.eq(other)
  }

  fn ge(&self, other: &Self) -> bool {
    self.gt(other) || self.eq(other)
  }

  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    if self.lt(other) {
      Some(Ordering::Less)
    } else if self.gt(other) {
      Some(Ordering::Greater)
    } else {
      Some(Ordering::Equal)
    }
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
      let a = Location { ln: 5, col: 5 };
      let b = Location { ln: 5, col: 4 };
      let c = Location { ln: 6, col: 5 };
      let d = Location { ln: 5, col: 5 };
      assert!(a > b);
      assert!(a < c);
      assert!(a == d);
      assert_eq!(a.partial_cmp(&b), Some(Ordering::Greater));
    }
}
