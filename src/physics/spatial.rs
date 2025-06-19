use num_bigint::BigInt;
use rust_decimal::Decimal;

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

////////////////////////////////////////
/// Speed is normalized on the speed of light
#[derive(Debug, Clone)]
pub struct Speed {
  negative_normalized: Decimal,
}

impl Speed {
  pub fn new_negative_normalized(speed: Decimal) -> Self {
    let max_speed: Decimal = Decimal::new(1, 0);
    let min_speed: Decimal = Decimal::new(-1, 0);

    // TODO: received value that falls outside the negative normalized bounds of -1 to 1
    if (speed < min_speed) || (speed > max_speed) { todo!() };

    Speed {
      negative_normalized : speed
    }
  }
  pub fn negative_normalized(&self) -> Decimal {
    self.negative_normalized.clone()
  }
}

////////////////////////////////////////
/// Velocity is speed in the direction of each axis
pub struct Velocity {
  //velocity: Vec<Speed>
}

