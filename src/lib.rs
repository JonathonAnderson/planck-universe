pub mod units;

pub enum Unit {
  Speed (units::Speed),
  Truth (units::Truth),
  Tick  (units::Tick),
}