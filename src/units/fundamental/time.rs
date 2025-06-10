use num_bigint::BigInt;

#[derive(Debug, Clone)]
pub struct Moment {
  units_from_origin: BigInt,

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
  fn new(units_from_origin: BigInt) -> Self {
    Moment {
      units_from_origin,

      dimension: crate::units::Dimension::Time,
      symbol: 'M',
    }
  }
  fn units_from_origin(&self) -> BigInt {
    self.units_from_origin.clone()
  }
}