use num_bigint::BigInt;

// Speed
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

impl crate::units::Unit for Point {
  fn dimension(&self) -> crate::units::Dimension {
    crate::units::Dimension::Space
  }
  fn symbol(&self) -> char {
    'P'
  }
}

// Space
const MAX: f64 = 1.0;
const MIN: f64 = -1.0;

#[derive(Debug)]
pub struct Speed {
  negative_normalized: f64,
}

impl Speed {
  pub fn new(negative_normalized: f64) -> Self {
    if (negative_normalized < MIN) || (negative_normalized > MAX) { todo!() };

    Speed {
      negative_normalized
    }
  }
  pub fn value(&self) -> f64 {
    self.negative_normalized
  }
}

impl crate::units::Unit for Speed {
  fn dimension(&self) -> crate::units::Dimension {
    crate::units::Dimension::Speed
  }
  fn symbol(&self) -> char {
    'S'
  }
}

// Time
#[derive(Debug, Clone)]
pub struct Moment {
  units_from_origin: BigInt
}

impl Moment {
  pub fn new(units_from_origin: BigInt) -> Self {
    Moment {
      units_from_origin,
    }
  }
  pub fn units_from_origin(&self) -> BigInt {
    self.units_from_origin.clone()
  }
}

impl crate::units::Unit for Moment {
  fn dimension(&self) -> crate::units::Dimension {
    crate::units::Dimension::Time
  }
  fn symbol(&self) -> char {
    'M'
  }
}