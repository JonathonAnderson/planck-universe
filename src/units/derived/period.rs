use crate::units::{Tick, FundamentalUnit, DerivedUnit };

#[derive(Debug)]
pub struct Period {
  beginning: Tick,
  end: Tick,

  dimension: crate::units::Dimension,
  symbol : char,
}

impl Period {
  // Ticks are moments on a time scale where 0 = beginning of time and 1 = end of time
  fn new(start_tick: &Tick, end_tick: &Tick) -> Self {
    Period {
      beginning : Tick::clone(start_tick),
      end : Tick::clone(end_tick),

      dimension : crate::units::Dimension::Time,
      symbol : 'P',
    }
  }
}

impl DerivedUnit for Period {
  fn value(&self) -> f64 {
    self.end.value() - self.beginning.value()
  }
  fn dimension(&self) -> crate::units::Dimension {
    self.dimension
  }
  fn symbol(&self) -> char {
    self.symbol
  }
}