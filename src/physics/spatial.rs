use num_bigint::BigInt;

////////////////////////////////////////
#[derive(Debug, Clone)]
pub struct Point {
  units_from_origin: Vec<BigInt>,
}

impl Point {
  pub fn new (units_from_origin: &Vec<BigInt>) -> Self {
    Point {
      units_from_origin : units_from_origin.clone(),
    }
  }
  pub fn units_from_origin(&self) -> Vec<BigInt> {
    self.units_from_origin.clone()
  }
  pub fn point(&self, axis: usize) -> BigInt {
    self.units_from_origin[axis].clone()
  }
}

////////////////////////////////////////
#[derive(Debug)]
pub struct Shape {
  points: Vec<Point>,
}

impl Shape {
  pub fn new( points: &Vec<&Point>) -> Self {
    Shape {
      points : points.iter().map(|&point| point.clone()).collect()
    }
  }
  pub fn points(&self) -> Vec<Point> {
    self.points.clone()
  }
}