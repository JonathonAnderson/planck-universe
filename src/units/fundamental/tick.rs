#[derive(Debug)]
pub struct Tick {
  period: f64,
  max: f64,
  min: f64,

  dimension: crate::units::Dimension,
  symbol : char,
}

impl crate::units::FundamentalUnit for Tick {
  fn new(value: f64) -> Self {
    Tick {
      period : value,
      max   : 1.0,
      min   : 0.0,

      dimension: crate::units::Dimension::Time,
      symbol: 'T',
    }
  }

  fn value(&self) -> f64 {
    self.period
  }
  fn set_value(self, value: f64) -> Result<Self, String> {
    if (value >= self.min) && (value <= self.max) {
      let new_value = Self{
        period : value,
        ..self
      };
      Ok(new_value)
    } else {
      Err("value must be above or equal to zero, start of period... AND... value must be below of equal to one, end of period".to_owned())
    }
  }
  fn dimension(&self) -> crate::units::Dimension {
    self.dimension
  }
  fn symbol(&self) -> char {
    self.symbol
  }
}