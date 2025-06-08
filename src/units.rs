pub trait Unit {
  fn new(value: f64) -> Self;
  fn value(&self) -> f64;
  fn set_value(self, value: f64) -> Result<Self, String>
    where
      Self: Sized;
  fn dimension(&self) -> Dimension;
  fn symbol(&self) -> char;
}

#[derive(Debug, Copy, Clone)]
pub enum Dimension {
  Speed,
  Truth,
  Time,
  Length,
}

mod speed;
pub use speed::Speed;

mod truth;
pub use truth::Truth;

mod tick;
pub use tick::Tick;

mod length;
pub use length::Length;