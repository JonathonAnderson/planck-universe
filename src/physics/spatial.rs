use num_bigint::BigInt;

////////////////////////////////////////
#[derive(Debug, Clone)]
pub struct Point {
  units_from_origin: Vec<BigInt>,
}

impl Point {
  pub fn new (units_from_origin: &[BigInt]) -> Self {
    Point {
      units_from_origin : units_from_origin.to_vec(),
    }
  }
  pub fn units_from_origin(&self) -> Vec<BigInt> {
    self.units_from_origin.clone()
  }
}

////////////////////////////////////////
#[derive(Debug)]
pub struct Shape {
  points: Vec<Point>,
}

impl Shape {
  pub fn new(points: &[Point]) -> Self {
    Shape {
      points : points.to_vec(),
    }
  }
  pub fn shape(&self) -> Vec<Point> {
    self.points.clone()
  }
}