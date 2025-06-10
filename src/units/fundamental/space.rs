use num_bigint::BigInt;

#[derive(Debug)]
pub struct Point {
  units_from_origin: BigInt,
}

impl crate::units::Unit for Point {
  fn dimension(&self) -> crate::units::Dimension {
    crate::units::Dimension::Space
  }
  fn symbol(&self) -> char {
    'P'
  }
}

impl crate::units::Discrete for Point {
  fn new(units_from_origin: BigInt) -> Self {
    Point {
      units_from_origin,
    }
  }
  fn units_from_origin(&self) -> BigInt {
    self.units_from_origin.clone()
  }
}