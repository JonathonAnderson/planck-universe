use num_bigint::BigInt;

#[derive(Debug)]
pub struct Point {
  // TODO: maybe this should be a vector of points to account for multi-dimensional points
  units_from_origin: Vec<BigInt>,
}

impl Point {
  pub fn new (units_from_origin: Vec<BigInt>) -> Self {
    Point {
      units_from_origin,
    }
  }
  pub fn units_from_origin(&self) -> Vec<BigInt> {
    self.units_from_origin.clone()
  }
}

impl crate::units::Unit for Point {
  fn dimension(&self) -> crate::units::Dimension {
    crate::units::Dimension::Space
  }
  fn symbol(&self) -> char {
    'P'
  }
}