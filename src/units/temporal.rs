use num_bigint::BigInt;

////////////////////////////////////////
#[derive(Debug, Clone)]
pub struct Moment {
  units_from_origin: BigInt
}

impl Moment {
  pub fn new(units_from_origin: BigInt) -> Self {
    Moment {
      units_from_origin,
    }
  }
  pub fn units_from_origin(&self) -> BigInt {
    self.units_from_origin.clone()
  }
}

impl crate::units::Unit for Moment {
  fn dimension(&self) -> crate::units::Dimension {
    crate::units::Dimension::Time
  }
  fn symbol(&self) -> char {
    'M'
  }
}

////////////////////////////////////////
#[derive(Debug)]
pub struct Duration {
  moment0: Moment,
  moment1: Moment,
}

impl Duration {
  pub fn new(moment0: &Moment, moment1: &Moment) -> Self {
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