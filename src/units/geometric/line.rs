use num_bigint::BigInt;

pub struct Line {
  point0: crate::units::fundamental::space::Point,
  point1: crate::units::fundamental::space::Point,
}

impl Line {
  pub fn new(point0: crate::units::fundamental::space::Point, point1: crate::units::fundamental::space::Point) -> Self {
    if point0.units_from_origin().len() != point1.units_from_origin().len() { todo!() }

    Line {
      point0,
      point1,
    }
  }
  pub fn length(&self) -> BigInt {
    let mut axis = 0;

    let point0_units: Vec<BigInt> = self.point0.units_from_origin();
    let point1_units: Vec<BigInt> = self.point1.units_from_origin();

    let mut length: BigInt = BigInt::from(0);

    while axis < point0_units.len() {
      let units0: &BigInt = point0_units.get(axis).unwrap();
      let units1: &BigInt = point1_units.get(axis).unwrap();

      length += (units0 - units1).pow(2);
      axis += 1;
    }

    length.sqrt()
  }
}

impl crate::units::Unit for Line {
  fn dimension(&self) -> crate::units::Dimension {
      crate::units::Dimension::Space
  }
  fn symbol(&self) -> char {
      'L'
  }
}