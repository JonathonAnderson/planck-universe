use crate::units::{self, Discrete};
use num_bigint::BigInt;
use num_traits::sign::Signed;

pub struct Length {
  point: crate::units::fundamental::space::Point,
  other_point: crate::units::fundamental::space::Point,
}

impl crate::units::Unit for Length {
  fn dimension(&self) -> units::Dimension {
      crate::units::Dimension::Space
  }
  fn symbol(&self) -> char {
      'D'
  }
}

impl crate::units::Length for Length {
  fn new(point: crate::units::fundamental::space::Point, other_point: crate::units::fundamental::space::Point) -> Self {
    Length {
      point,
      other_point,
    }
  }
  fn length(&self) -> BigInt {
    (self.point.units_from_origin() - self.other_point.units_from_origin()).abs()
  }
}