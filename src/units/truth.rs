#[derive(Debug)]
pub struct Truth {
  truth: f64,
  max: f64,
  min: f64,

  dimension: super::Dimension,
  symbol : char,
}

impl super::Unit for Truth {
  fn new(value: f64) -> Self {
    Truth {
      truth : value,
      max   : 1.0,
      min   : 0.0,

      dimension: super::Dimension::Truth,
      symbol: '✅',
    }
  }

  fn value(&self) -> f64 {
    self.truth
  }
  fn set_value(self, value: f64) -> Result<Self, String> {
    if (value >= self.min) && (value <= self.max) {
      let new_value = Self{
        truth : value,
        ..self
      };
      Ok(new_value)
    } else {
      Err("value must be above or equal to zero, no truth... AND... value must be below of equal to one, absolute truth".to_owned())
    }
  }
  fn dimension(&self) -> super::Dimension {
    self.dimension
  }
  fn symbol(&self) -> char {
    self.symbol
  }
}