use num_bigint::BigInt;

////////////////////////////////////////
#[derive(Debug, Clone)]
pub struct Point {
  units_from_origin: Vec<BigInt>,
}

impl Point {
  pub fn new (units_from_origin: Vec<BigInt>) -> Self {
    Point {
      units_from_origin,
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
const MIN_ORIENTATION: f64 = -1.0;
const MAX_ORIENTATION: f64 = 1.0;

pub struct Orientation {}

impl Orientation {
  pub fn new(point: Point, orientation: Vec<f64>) -> Self {
    if orientation.len() != point.units_from_origin.len() { todo!() }

    for orientation in &orientation {
      if orientation < &MIN_ORIENTATION || orientation > &MAX_ORIENTATION { todo!() }
    }

    Orientation {}
  }
}

////////////////////////////////////////
const MAX_SPEED: f64 = 1.0;
const MIN_SPEED: f64 = -1.0;

#[derive(Debug, Clone)]
pub struct Speed {
  negative_normalized: f64,
}

impl Speed {
  pub fn new(negative_normalized: f64) -> Self {
    // TODO: received value that falls outside the negative normalized bounds of -1 to 1
    if (negative_normalized < MIN_SPEED) || (negative_normalized > MAX_SPEED) { todo!() };

    Speed {
      negative_normalized
    }
  }
  pub fn negative_normalized(&self) -> f64 {
    self.negative_normalized
  }
}

////////////////////////////////////////
pub struct Velocity {
  // speed: Speed,
  // orientation: Orientation,
}

impl Velocity {
  pub fn new() -> Self {
    Velocity {}
  }
}

////////////////////////////////////////
#[derive(Debug)]
pub struct Shape {
  points: Vec<Point>,
}

impl Shape {
  pub fn new( points: Vec<Point>) -> Self {
    Shape {
      points
    }
  }
  pub fn points(&self) -> Vec<Point> {
    self.points.clone()
  }
}

////////////////////////////////////////
#[derive(Debug)]
pub struct Line {
  point0: Point,
  point1: Point,
  length: BigInt,
}

impl Line {
  pub fn new(point0: &Point, point1: &Point) -> Self {
    // TODO: Received points with different number of axes
    if point0.units_from_origin().len() != point1.units_from_origin().len() { todo!() }

    let point0 = point0.clone();
    let point1 = point1.clone();
    let length =  {
                    let mut axis = 0;

                    let point0_units: Vec<BigInt> = point0.units_from_origin();
                    let point1_units: Vec<BigInt> = point1.units_from_origin();

                    let mut length: BigInt = BigInt::from(0);

                    while axis < point0_units.len() {
                      let units0: &BigInt = point0_units.get(axis).unwrap();
                      let units1: &BigInt = point1_units.get(axis).unwrap();

                      length += (units0 - units1).pow(2);
                      axis += 1;
                    }

                    length.sqrt()
                  };

    Line {
      point0,
      point1,
      length,
    }
  }

  pub fn point0(&self) -> Point{
    self.point0.clone()
  }
  pub fn point1(&self) -> Point {
    self.point1.clone()
  }
  pub fn length(&self) -> BigInt {
    self.length.clone()
  }
}