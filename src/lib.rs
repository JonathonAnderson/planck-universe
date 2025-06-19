use num_bigint::BigInt;
use rust_decimal::Decimal;
use crate::physics::temporal::{Moment, Duration};
use crate::physics::spatial::{Point, Speed};

pub mod physics;

#[derive(Debug, Clone)]
pub struct Object {
  t_0: Moment,
  t_final: Moment,

  geometry: Vec<Point>,

  speed: Speed,
}

impl Default for Object {
  fn default() -> Self {

    let t_0 = Moment::new(BigInt::from(0));
    let t_final = Moment::new(BigInt::from(0));
    let geometry = [].to_vec();
    let speed = Speed::new_negative_normalized(Decimal::new(0,0));

    Self{
      t_0,
      t_final,
      geometry,
      speed,
    }
  }
}

impl Object {
  pub fn new() -> Self {
    Self {
      ..Default::default()
    }
  }
  
  pub fn t_0(&self) -> Moment {
    self.t_0.clone()
  }

  pub fn set_t_0(self, moment: &Moment) -> Self {
    Self {
      t_0 : moment.clone(),
      ..self.clone()
    }
  }

  pub fn t_final(&self) -> Moment {
    self.t_final.clone()
  }

  pub fn set_t_final(self, moment: &Moment) -> Self {
    Self {
      t_final : moment.clone(),
      ..self.clone()
    }
  }

  pub fn duration(self) -> Duration {
    Duration::new(&self.t_final, &self.t_0)
  }

  pub fn set_duration(self, duration: Duration) -> Self {
    Self {
      t_0 : duration.begin(),
      t_final : duration.end(),
      ..self.clone()
    }
  }

  pub fn normalized_speed(self) -> Speed {
    Speed::new_negative_normalized(self.speed.negative_normalized())
  }

  pub fn set_normalized_speed(self, speed: Speed) -> Self {
    Object {
      speed,
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