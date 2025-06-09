pub mod units;

pub enum Unit {
  Speed  (units::Speed),
  Truth  (units::Truth),
  Tick   (units::Tick),
  Period (units::Period),
  Length (units::Length),
}

pub struct Object {
  units : Vec<Unit>,
}