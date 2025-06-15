use num_bigint::BigInt;

#[derive(Debug)]
pub struct Duration {
  moment0: crate::units::fundamental::time::Moment,
  moment1: crate::units::fundamental::time::Moment,
}

impl Duration {
  pub fn new(moment0: &crate::units::fundamental::time::Moment, moment1: &crate::units::fundamental::time::Moment) -> Self {
    Duration {
      moment0 : moment0.clone(),
      moment1 :moment1.clone(),
    }
  }
  pub fn duration(&self) -> BigInt {
    self.moment0.units_from_origin() - self.moment1.units_from_origin()
  }
}

impl crate::units::Unit for Duration {
  fn dimension(&self) -> crate::units::Dimension {
      crate::units::Dimension::Time
  }
  fn symbol(&self) -> char {
      'D'
  }
}