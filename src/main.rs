mod api;
mod battery;
mod cli;
mod last_battery;
mod net;
mod notification;

use std::{error::Error, process::ExitCode, thread, time::Duration};

use battery::Battery;
use cli::Cli;
use notification::Notification;

const BATTERY_LIFETIME_HOURS: u32 = 7;
const DELAY: Duration = Duration::from_secs(42);

const BIN_NAME: &str = env!("CARGO_BIN_NAME");

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> ExitCode {
  match launch() {
    Ok(()) => ExitCode::SUCCESS,
    Err(err) => {
      eprintln!("{err}");
      ExitCode::FAILURE
    }
  }
}

fn launch() -> Result<()> {
  match Cli::parse() {
    Cli::Info => info()?,
    Cli::Watch => loop {
      info()?;
      thread::sleep(DELAY);
    },
    Cli::Charge => charge()?,
    Cli::Reboot => {
      api::reboot(&api::login()?)?;
      Notification::common("Rebooting ..").show()?;
    }
    Cli::Off => {
      api::off(&api::login()?)?;
      Notification::common("Switching off ..").show()?;
    }
  }
  Ok(())
}

fn info() -> Result<()> {
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

fn charge() -> Result<()> {
  Notification::common("Will be off on charged (Swipe to cancel)")
    .on_delete(&format!(r#"pkill -f "{BIN_NAME} charge""#))
    .show()?;
  let mut prev: Option<Battery> = None;
  loop {
    let auth_cookie = &api::login()?;
    let battery = api::battery(auth_cookie)?;
    if charged_enough(&battery) || charging_disabled(prev, &battery) {
      api::off(auth_cookie)?;
      Notification::common("Charged and off ..").show()?;
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
  prev.is_some_and(|b| b.charging) && !battery.charging
}
