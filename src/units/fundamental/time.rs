use num_bigint::BigInt;

#[derive(Debug, Clone)]
pub struct Moment {
  value: BigInt,

  dimension: crate::units::Dimension,
  symbol : char,
}

impl crate::units::Unit for Moment {
  fn dimension(&self) -> crate::units::Dimension {
    self.dimension.clone()
  }
  fn symbol(&self) -> char {
    self.symbol
  }
}

impl crate::units::Discrete for Moment {
  fn new(value: BigInt) -> Self {
    Moment {
      value,

      dimension: crate::units::Dimension::Time,
      symbol: 'T',
    }
  }
  fn value(&self) -> BigInt {
    self.value.clone()
  }
}