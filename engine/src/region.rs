use std::fmt;
use std::fmt::Display;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Region {
  left: usize,
  top: usize,
  width: usize,
  height: usize,
}

impl Display for Region {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "({}, {}, {}, {})", self.left, self.top, self.width, self.height)
  }
}

impl Region {
  pub fn new(left: usize, top: usize, width: usize, height: usize) -> Self {
    Self { left, top, width, height }
  }

  pub fn left(&self) -> usize {
    self.left
  }

  pub fn right(&self) -> usize {
    self.left.saturating_add(self.width.saturating_sub(1))
  }

  pub fn top(&self) -> usize {
    self.top
  }

  pub fn bottom(&self) -> usize {
    self.top.saturating_add(self.height.saturating_sub(1))
  }

  pub fn width(&self) -> usize {
    self.width
  }

  pub fn height(&self) -> usize {
    self.height
  }

  pub fn offset(&self) -> (usize, usize) {
    (self.left, self.top)
  }

  pub fn size(&self) -> (usize, usize) {
    (self.width, self.height)
  }

  pub fn resize(&mut self, width: usize, height: usize) {
    self.width = width;
    self.height = height;
  }

  pub fn clip(&self, other: &Region) -> Region {
    let left = if other.left() > self.left() { other.left() } else { self.left() };
    let top = if other.top() > self.top() { other.top() } else { self.top() };
    let right = if other.right() < self.right() { other.right() } else { self.right() };
    let bottom = if other.bottom() < self.bottom() { other.bottom() } else { self.bottom() };
    let width = right.saturating_sub(left).saturating_add(1);
    let height = bottom.saturating_sub(top).saturating_add(1);
    Region { left, top, width, height }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn debug_should_work() {
    let region = Region::new(10, 20, 100, 200);
    assert_eq!("Region { left: 10, top: 20, width: 100, height: 200 }", format!("{:?}", region));
  }

  #[test]
  fn display_should_work() {
    let region = Region::new(10, 20, 100, 200);
    assert_eq!("(10, 20, 100, 200)", format!("{}", region));
  }

  #[test]
  fn clipping_should_work() {
    let viewport = Region::new(0, 0, 300, 200);
    let changed = Region::new(90, 10, 10, 100);
    let clipped = Region::new(90, 10, 10, 100);
    assert_eq!(clipped, changed.clip(&viewport));
    let viewport = Region::new(10, 10, 200, 200);
    let changed = Region::new(180, 0, 120, 300);
    let clipped = Region::new(180, 10, 30, 200);
    assert_eq!(clipped, changed.clip(&viewport));
  }
}
