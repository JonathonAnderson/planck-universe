const MAX: f64 = 1.0;
const MIN: f64 = -1.0;

#[derive(Debug)]
pub struct Speed {
  value: f64,
}

impl crate::units::Unit for Speed {
  fn dimension(&self) -> crate::units::Dimension {
    crate::units::Dimension::Speed
  }
  fn symbol(&self) -> char {
    'S'
  }
}

impl crate::units::SignedNormalized for Speed {
  fn new(value: f64) -> Self {
    if (value < MIN) || (value > MAX) { todo!() };

    Speed {
      value
    }
  }
  fn value(&self) -> f64 {
    self.value
  }
}