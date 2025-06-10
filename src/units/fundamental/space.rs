use num_bigint::BigInt;

#[derive(Debug)]
pub struct Point {
  value: BigInt,

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
  fn new(value: BigInt) -> Self {
    Point {
      value,

      dimension: crate::units::Dimension::Time,
      symbol: 'T',
    }
  }
  fn value(&self) -> BigInt {
    self.value.clone()
  }
}