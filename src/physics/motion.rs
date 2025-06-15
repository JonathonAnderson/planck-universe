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