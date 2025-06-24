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
#[derive(Debug, Clone)]
pub struct Time {
  moment0: Moment,
  moment1: Moment,
  time: BigInt,
}

impl Time {
  pub fn new(moment0: &Moment, moment1: &Moment) -> Self {
    
    let moment0 = moment0.clone();
    let moment1 = moment1.clone();
    let time = moment1.units_from_origin.clone() - moment0.units_from_origin.clone();

    Time {
      moment0,
      moment1,
      time,
    }

  }
  pub fn begin(&self) -> Moment {
    self.moment0.clone()
  }
  pub fn end(&self) -> Moment {
    self.moment1.clone()
  }
  pub fn time(&self) -> BigInt {
    self.time.clone()
  }
}

////////////////////////////////////////
#[derive(Debug)]
pub struct Period {
  time0: Time,
  time1: Time,
  time:  BigInt,
}

impl Period {
  pub fn new(time0: &Time, time1: &Time) -> Self {
    
    let time0 = time0.clone();
    let time1 = time1.clone();
    let time  = &time0.time + &time1.time;

    Period {
      time0,
      time1,
      time,
    }

  }
  pub fn begin(&self) -> Time {
    self.time0.clone()
  }
  pub fn end(&self) -> Time {
    self.time1.clone()
  }
  pub fn time(&self) -> BigInt {
    self.time.clone()
  }
}