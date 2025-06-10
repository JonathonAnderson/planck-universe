use num_bigint::BigInt;

pub trait Unit {
  fn dimension(&self) -> Dimension;
  fn symbol(&self) -> char;
}

pub trait SignedNormalized {
  fn new(value: f64) -> Self;
  fn value(&self) -> f64;
}

pub trait Discrete {
  fn new(value: BigInt) -> Self;
  fn units_from_origin(&self) -> BigInt;
}

pub trait Length {
  fn new(point: fundamental::space::Point, other_point: fundamental::space::Point) -> Self;
  fn length(&self) -> BigInt;
}

pub trait Duration {
  fn new(point: fundamental::time::Moment, other_point: fundamental::time::Moment) -> Self;
  fn duration(&self) -> BigInt;
}

#[derive(Debug, Clone)]
pub enum Dimension {
  Speed,
  Time,
  Space,
}

mod fundamental;

mod derived;

pub use fundamental::speed::Speed;
pub use fundamental::time::Moment;
pub use fundamental::space::Point;

// pub use derived::period::Period;
// pub use derived::length::Length;