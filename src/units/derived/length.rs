use crate::units::{ fundamental::speed::Speed, FundamentalUnit, DerivedUnit };
use crate::units::derived::period::Period;

#[derive(Debug)]
pub struct Length {
  length: f64,

  dimension: crate::units::Dimension,
  symbol : char,
}

impl Length {
  // Ticks are moments on a time scale where 0 = beginning of time and 1 = end of time
  fn new(speed: &Speed, period: &Period) -> Self {
    Length {
      length : speed.value() * period.value(),

      dimension : crate::units::Dimension::Length,
      symbol : 'L',
    }
  }
}

impl DerivedUnit for Length {
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