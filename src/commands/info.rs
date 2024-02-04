use crate::{api, last_battery, notification::Notification, Result};

pub(crate) fn info() -> Result<()> {
  let Ok(ref auth_cookie) = api::login() else {
    let battery_status =
      last_battery_short_status().unwrap_or_default();
    Notification::ongoing(format!("Disconnected {battery_status}"))
      .show()?;
    return Ok(());
  };
  let battery = api::battery(auth_cookie)?;
  let net = api::net(auth_cookie)?;
  Notification::ongoing(format!("{battery}\t\t{net}"))
    .set_power_buttons()
    .show()?;
  last_battery::set(battery);
  Ok(())
}

fn last_battery_short_status() -> Option<String> {
  let battery = last_battery::get()?;
  let badge = battery.badge();
  let capacity = battery.capacity;
  Some(format!("{badge} ~{capacity}%"))
}
