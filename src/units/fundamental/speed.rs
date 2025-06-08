#[derive(Debug)]
pub struct Speed {
  speed: f64,
  max: f64,
  min: f64,

  dimension: crate::units::Dimension,
  symbol : char,
}

impl crate::units::FundamentalUnit for Speed {
  fn new(value: f64) -> Self {
    Speed {
      speed : value,
      max   : 1.0,
      min   : 0.0,

      dimension: crate::units::Dimension::Speed,
      symbol: 'S',
    }
  }

  fn value(&self) -> f64 {
    self.speed
  }
  fn set_value(self, value: f64) -> Result<Self, String> {
    if (value >= self.min) && (value <= self.max) {
      let new_value = Self{
        speed : value,
        ..self
      };
      Ok(new_value)
    } else {
      Err("value must be above or equal to zero, no motion... AND... value must be below of equal to one, the value of light".to_owned())
    }
  }
  fn dimension(&self) -> crate::units::Dimension {
    self.dimension
  }
  fn symbol(&self) -> char {
    self.symbol
  }
}