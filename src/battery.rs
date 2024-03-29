use std::{fmt, time::Duration};

use crate::BATTERY_LIFETIME;

const HOUR: Duration = Duration::from_secs(3600);

#[derive(Debug, Clone)]
pub(crate) struct Battery {
  pub(crate) capacity: u8,
  pub(crate) voltage: f32,
  pub(crate) charging: bool,
}

impl Battery {
  pub(crate) fn badge(&self) -> &'static str {
    match () {
      () if self.charging => "⚡️",
      () if self.capacity > 15 => "🔋",
      () => "🪫",
    }
  }

  fn lifetime(&self) -> Duration {
    BATTERY_LIFETIME * self.capacity.into() / 100
  }

  pub(crate) fn charged_enough(&self) -> bool {
    self.capacity > 90
  }
}

impl fmt::Display for Battery {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let badge = self.badge();
    let capacity = self.capacity;
    let voltage = self.voltage;
    let lifetime = to_time_string(self.lifetime());
    write!(f, "{badge} {capacity}% {voltage}V ~{lifetime}")
  }
}

fn to_time_string(dur: Duration) -> String {
  let h = dur.as_secs_f32() / HOUR.as_secs_f32();
  if h > 1. {
    let h = (h * 10.).floor() / 10.;
    format!("{h}h")
  } else {
    let m = (h * 60.).floor();
    format!("{m}m")
  }
}
