#[derive(Debug)]
pub struct Length {
  length: f64,
  max: f64,
  min: f64,

  dimension: super::Dimension,
  symbol : char,
}

impl super::Unit for Length {
  fn new(value: f64) -> Self {
    Length {
      length : value,
      max   : 1.0,
      min   : 0.0,

      dimension: super::Dimension::Length,
      symbol: 'L',
    }
  }

  fn value(&self) -> f64 {
    self.length
  }
  fn set_value(self, value: f64) -> Result<Self, String> {
    if (value >= self.min) && (value <= self.max) {
      let new_value = Self{
        length : value,
        ..self
      };
      Ok(new_value)
    } else {
      Err("value must be above or equal to zero, start point... AND... value must be below of equal to one, end point".to_owned())
    }
  }
  fn dimension(&self) -> super::Dimension {
    self.dimension
  }
  fn symbol(&self) -> char {
    self.symbol
  }
}