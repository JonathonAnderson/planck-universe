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

////////////////////////////////////////
#[derive(Debug)]
pub struct Duration {
  moment0: Moment,
  moment1: Moment,
  duration: BigInt,
}

impl Duration {
  pub fn new(moment2: &Moment, moment3: &Moment) -> Self {
    
    let moment0 = moment2.clone();
    let moment1 = moment3.clone();
    let duration = moment0.units_from_origin.clone() - moment1.units_from_origin.clone();

    Duration {
      moment0,
      moment1,
      duration,
    }

  }
  pub fn begin(&self) -> Moment {
    self.moment0.clone()
  }
  pub fn end(&self) -> Moment {
    self.moment1.clone()
  }
  pub fn duration(&self) -> BigInt {
    self.duration.clone()
  }
}