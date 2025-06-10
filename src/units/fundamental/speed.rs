const MAX: f64 = 1.0;
const MIN: f64 = -1.0;

#[derive(Debug)]
pub struct Speed {
  value: f64,

  dimension: crate::units::Dimension,
  symbol : char,
}

impl crate::units::Unit for Speed {
  fn dimension(&self) -> crate::units::Dimension {
    self.dimension.clone()
  }
  fn symbol(&self) -> char {
    self.symbol
  }
}

impl crate::units::SignedNormalized for Speed {
  fn new(value: f64) -> Self {
    if (value < MIN) || (value > MAX) { todo!() };

    Speed {
      value,

      dimension: crate::units::Dimension::Speed,
      symbol: 'S',
    }
  }
  fn value(&self) -> f64 {
    self.value
  }
}