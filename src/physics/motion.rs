use rust_decimal::Decimal;

/// Speed is normalized on the speed of light
/// and allows negatives just to see what happens
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

