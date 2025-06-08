use crate::units::{ fundamental::speed::Speed, FundamentalUnit };

#[derive(Debug)]
pub struct Length {
  length: f64,

  dimension: crate::units::Dimension,
  symbol : char,
}

impl Length {
  fn new(speed: &Speed, tick_count: u128) -> Self {
    Length {
      length : speed.value() * (tick_count as f64),

      dimension : crate::units::Dimension::Length,
      symbol : 'L',
    }
  }
}

impl crate::units::DerivedUnit for Length {
  fn value(&self) -> f64 {
    self.length
  }
  fn dimension(&self) -> crate::units::Dimension {
    self.dimension
  }
  fn symbol(&self) -> char {
    self.symbol
  }
}