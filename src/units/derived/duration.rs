use crate::units::{self, Discrete};
use num_bigint::BigInt;
use num_traits::sign::Signed;

pub struct Duration {
  moment: crate::units::fundamental::time::Moment,
  other_moment: crate::units::fundamental::time::Moment,
}

impl crate::units::Unit for Duration {
  fn dimension(&self) -> units::Dimension {
      crate::units::Dimension::Time
  }
  fn symbol(&self) -> char {
      'D'
  }
}

impl crate::units::Duration for Duration {
  fn new(moment: crate::units::fundamental::time::Moment, other_moment: crate::units::fundamental::time::Moment) -> Self {
    Duration {
      moment,
      other_moment,
    }
  }
  fn duration(&self) -> BigInt {
    (self.moment.units_from_origin() - self.other_moment.units_from_origin()).abs()
  }
}