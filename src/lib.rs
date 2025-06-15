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

impl Default for Object {
  fn default() -> Self {
    Self{
      begin_existence : Moment::new(BigInt::from(0)),
      end_existence : Moment::new(BigInt::from(0)),
      geometry : [].to_vec(),
      normalized_speed : Speed::new(0.0),
    }
  }
}

impl Object {
  pub fn new() -> Self {
    Self {
      ..Default::default()
    }
  }

  pub fn speed(&self) -> Speed {
    self.normalized_speed.clone()
  }

  pub fn set_speed(self, new_speed: Speed) -> Self {
    Self {
      normalized_speed : new_speed,
      ..Default::default()
    }
  }
  
  pub fn t_0(&self) -> Moment {
    self.begin_existence.clone()
  }

  pub fn set_t_0(self, moment: &Moment) -> Self {
    Self {
      begin_existence : moment.clone(),
      ..self.clone()
    }
  }

  pub fn t_final(&self) -> Moment {
    self.end_existence.clone()
  }

  pub fn set_t_final(self, moment: &Moment) -> Self {
    Self {
      end_existence : moment.clone(),
      ..self.clone()
    }
  }

  pub fn duration(self) -> Duration {
    Duration::new(&self.end_existence, &self.begin_existence)
  }

  pub fn set_duration(self, duration: Duration) -> Self {
    Self {
      begin_existence : duration.begin(),
      end_existence : duration.end(),
      ..self.clone()
    }
  }

  pub fn normalized_speed(self) -> Speed {
    Speed::new(self.normalized_speed.negative_normalized())
  }

  pub fn set_normalized_speed(self, speed: Speed) -> Self {
    Object {
      normalized_speed : speed,
      ..Default::default()
    }
  }

  pub fn geometry(self) -> Vec<Point> {
    self.geometry
  }

  pub fn set_geometry(self, geometry: Vec<Point>) -> Self {
      Self {
        geometry : geometry.clone(),
        ..self.clone()
      }
  }
}