use num_bigint::BigInt;
use crate::physics::temporal::Moment;
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

  pub fn begin_existence(&self) -> Moment {
    self.begin_existence.clone()
  }

  pub fn end_existence(&self) -> Moment {
    self.end_existence.clone()
  }

  pub fn geometry(&self) -> Vec<Point> {
    self.geometry.clone()
  }

  pub fn normalized_speed(&self) -> Speed {
    self.normalized_speed.clone()
  }
}