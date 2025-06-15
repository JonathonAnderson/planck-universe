pub mod temporal;
pub mod geometric;

// Speed
const MAX_SPEED: f64 = 1.0;
const MIN_SPEED: f64 = -1.0;

#[derive(Debug)]
pub struct Speed {
  negative_normalized: f64,
}

impl Speed {
  pub fn new(negative_normalized: f64) -> Self {
    if (negative_normalized < MIN_SPEED) || (negative_normalized > MAX_SPEED) { todo!() };

    Speed {
      negative_normalized
    }
  }
  pub fn value(&self) -> f64 {
    self.negative_normalized
  }
}