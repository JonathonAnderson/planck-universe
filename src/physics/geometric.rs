use num_bigint::BigInt;

////////////////////////////////////////
#[derive(Debug, Clone)]
pub struct Point {
  // TODO: maybe this should be a vector of points to account for multi-dimensional points
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