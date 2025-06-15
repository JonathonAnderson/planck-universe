use physics::{temporal::*, geometric::*, motion::*};

pub mod physics;

#[derive(Debug, Clone)]
pub struct Object {
  begin_existence: Moment,
  end_existence: Moment,

  geometry: Vec<Point>,

  normalized_speed: Speed,
}

impl Object {
  pub fn new(begin_existence: &Moment, end_existence: &Moment, geometry: &Vec<Point>, normalized_speed: &Speed) -> Self {
    Object {
      begin_existence: begin_existence.clone(),
      end_existence: end_existence.clone(),
      geometry: geometry.clone(),
      normalized_speed: normalized_speed.clone(),
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