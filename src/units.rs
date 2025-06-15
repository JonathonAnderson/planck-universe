pub mod fundamental;
pub mod temporal;
pub mod geometric;

pub trait Unit {
  fn dimension(&self) -> Dimension;
  fn symbol(&self) -> char;
}

#[derive(Debug, Clone)]
pub enum Dimension {
  Speed,
  Time,
  Space,
}