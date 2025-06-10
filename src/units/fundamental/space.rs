use num_bigint::BigInt;

#[derive(Debug)]
pub struct Point {
  units_from_origin: BigInt,

  dimension: crate::units::Dimension,
  symbol : char,
}

impl crate::units::Unit for Point {
  fn dimension(&self) -> crate::units::Dimension {
    self.dimension.clone()
  }
  fn symbol(&self) -> char {
    self.symbol
  }
}

impl crate::units::Discrete for Point {
  fn new(units_from_origin: BigInt) -> Self {
    Point {
      units_from_origin,

      dimension: crate::units::Dimension::Space,
      symbol: 'P',
    }
  }
  fn units_from_origin(&self) -> BigInt {
    self.units_from_origin.clone()
  }
}