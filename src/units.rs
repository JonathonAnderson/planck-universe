pub trait FundamentalUnit {
  fn new(value: f64) -> Self;
  fn value(&self) -> f64;
  fn set_value(self, value: f64) -> Result<Self, String>
    where
      Self: Sized;
  fn dimension(&self) -> Dimension;
  fn symbol(&self) -> char;
}

pub trait DerivedUnit {
  fn value(&self) -> f64;
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

mod fundamental;

mod derived;

pub use fundamental::speed::Speed;
pub use fundamental::tick::Tick;
pub use fundamental::truth::Truth;

pub use derived::length::Length;