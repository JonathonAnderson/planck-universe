use num_bigint::BigInt;
use crate::physics::temporal::{Moment, Duration};
use crate::physics::spatial::{Point, Speed};

pub mod physics;

#[derive(Debug, Clone)]
pub struct Object {
  begin_existence: Moment,
  end_existence: Moment,

  geometry: Vec<Point>,

  normalized_speed: Speed,
}

impl Object {
  pub fn new() -> Self {
    Self {
      begin_existence  : Moment::new(BigInt::from(0)),
      end_existence    : Moment::new(BigInt::from(0)),
      geometry         : vec![],
      normalized_speed : Speed::new(0.0),
     }
  }

  pub fn set_begin_existence(&self, moment: &Moment) -> Self {
    Self {
      begin_existence : moment.clone(),
      end_existence : self.end_existence.clone(),
      geometry : self.geometry.clone(),
      normalized_speed : self.normalized_speed.clone(),
    }
  }

  pub fn begin_existence(&self) -> Moment {
    self.begin_existence.clone()
  }

  pub fn set_end_existence(self, moment: &Moment) -> Self {
    Self {
      begin_existence : self.begin_existence,
      end_existence : moment.clone(),
      geometry : self.geometry,
      normalized_speed : self.normalized_speed,
    }
  }

  pub fn set_duration(self, duration: Duration) -> Self {
    Object {
      begin_existence : duration.begin(),
      end_existence : duration.end(),
      geometry : self.geometry,
      normalized_speed : self.normalized_speed,
    }
  }

  pub fn end_existence(&self) -> Moment {
    self.end_existence.clone()
  }

  pub fn set_geometry(&self) -> Vec<Point> {
    self.geometry.clone()
  }

  pub fn normalized_speed(&self) -> Speed {
    self.normalized_speed.clone()
  }
}