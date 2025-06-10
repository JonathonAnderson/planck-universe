use num_bigint::BigInt;

pub struct Length {
  point: crate::units::fundamental::space::Point,
  other_point: crate::units::fundamental::space::Point,
}

impl crate::units::Unit for Length {
  fn dimension(&self) -> crate::units::Dimension {
      crate::units::Dimension::Space
  }
  fn symbol(&self) -> char {
      'D'
  }
}

impl crate::units::Length for Length {
  fn new(point: crate::units::fundamental::space::Point, other_point: crate::units::fundamental::space::Point) -> Self {
    if point.units_from_origin().len() != other_point.units_from_origin().len() { todo!() }

    Length {
      point,
      other_point,
    }
  }
  fn length(&self) -> BigInt {
    let mut axis = 0;

    let point_units: Vec<BigInt> = self.point.units_from_origin();
    let other_point_units: Vec<BigInt> = self.other_point.units_from_origin();

    let mut length: BigInt = BigInt::from(0);

    while axis < point_units.len() {
      let point = point_units.get(axis).unwrap();
      let other_point = other_point_units.get(axis).unwrap();

      length += (point - other_point).pow(2);
      axis += 1;
    }

    length.sqrt()
  }
}