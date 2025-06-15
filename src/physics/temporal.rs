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
pub struct Duration {
  moment0: Moment,
  moment1: Moment,
  duration: BigInt,
}

impl Duration {
  pub fn new(moment0: &Moment, moment1: &Moment) -> Self {
    
    let moment0 = moment0.clone();
    let moment1 = moment1.clone();
    let duration = moment1.units_from_origin.clone() - moment0.units_from_origin.clone();

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

////////////////////////////////////////
#[derive(Debug)]
pub struct Period {
  duration0: Duration,
  duration1: Duration,
  duration:  BigInt,
}

impl Period {
  pub fn new(duration0: &Duration, duration1: &Duration) -> Self {
    
    let duration0 = duration0.clone();
    let duration1 = duration1.clone();
    let duration  = &duration1.duration - &duration0.duration;

    Period {
      duration0,
      duration1,
      duration,
    }

  }
  pub fn begin(&self) -> Duration {
    self.duration0.clone()
  }
  pub fn end(&self) -> Duration {
    self.duration1.clone()
  }
  pub fn duration(&self) -> BigInt {
    self.duration.clone()
  }
}