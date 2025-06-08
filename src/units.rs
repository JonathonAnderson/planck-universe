pub enum Unit {
  Speed (Speed),
}

#[derive(Debug)]
pub struct Speed {
  speed: f64,
  max: f64,
  min: f64,

  unit: String,
  symbol : char,
}

impl Speed {
  pub fn new(speed: f64) -> Self {
    Self {
      speed,
      max : 1.0,
      min : 0.0,

      unit: "SPEED".to_owned(),
      symbol: 'S',
    }
  }

  pub fn get_speed(&self) -> f64 {
    self.speed
  }

  pub fn set_speed(self, speed: f64) -> Result<Self, String> {
    if (speed >= self.min) && (speed <= self.max) {
      let new_speed = Speed{
        speed,
        ..self
      };
      Ok(new_speed)
    } else {
      Err("Speed must be above or equal to zero, no motion... AND... Speed must be below of equal to one, the speed of light".to_owned())
    }
  }
}