use std::thread;

use crate::{
  api, battery::Battery, notification::Notification, Result,
  BIN_NAME, DELAY,
};

pub(crate) fn charge() -> Result<()> {
  Notification::common("Will be off on charged (Swipe to cancel)")
    .on_delete(&format!(r#"pkill -f "{BIN_NAME} charge""#))
    .show()?;
  let mut prev = None;
  loop {
    let auth_cookie = &api::login()?;
    let battery = api::battery(auth_cookie)?;
    if charged_enough(&battery) {
      api::off(auth_cookie)?;
      Notification::common("Charged and off ..").show()?;
      return Ok(());
    }
    if charging_disabled(prev, &battery) {
      api::off(auth_cookie)?;
      Notification::common("Charging stopped ..").show()?;
      return Ok(());
    }
    prev = Some(battery);
    thread::sleep(DELAY);
  }
}

fn charged_enough(battery: &Battery) -> bool {
  battery.charging && battery.capacity > 92
}

fn charging_disabled(
  prev: Option<Battery>,
  battery: &Battery,
) -> bool {
  prev.is_some_and(|prev| prev.charging) && !battery.charging
}
