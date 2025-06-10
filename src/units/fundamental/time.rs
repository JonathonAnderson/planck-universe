use num_bigint::BigInt;

#[derive(Debug, Clone)]
pub struct Moment {
  units_from_origin: BigInt
}

impl crate::units::Unit for Moment {
  fn dimension(&self) -> crate::units::Dimension {
    crate::units::Dimension::Time
  }
  fn symbol(&self) -> char {
    'M'
  }
}

impl crate::units::Discrete for Moment {
  fn new(units_from_origin: BigInt) -> Self {
    Moment {
      units_from_origin,
    }
  }
  fn units_from_origin(&self) -> BigInt {
    self.units_from_origin.clone()
  }
}