pub mod temporal;
pub mod geometric;

// Speed
const MAX: f64 = 1.0;
const MIN: f64 = -1.0;

#[derive(Debug)]
pub struct Speed {
  negative_normalized: f64,
}

impl Speed {
  pub fn new(negative_normalized: f64) -> Self {
    if (negative_normalized < MIN) || (negative_normalized > MAX) { todo!() };

    Speed {
      negative_normalized
    }
  }
  pub fn value(&self) -> f64 {
    self.negative_normalized
  }
}

impl crate::units::Unit for Speed {
  fn dimension(&self) -> crate::units::Dimension {
    crate::units::Dimension::Speed
  }
  fn symbol(&self) -> char {
    'S'
  }
}

pub trait Unit {
  fn dimension(&self) -> Dimension;
  fn symbol(&self) -> char;
}

#[derive(Debug, Clone)]
pub enum Dimension {
  Speed,
  Time,
  Space,
}