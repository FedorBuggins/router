mod api;
mod battery;
mod cli;
mod last_battery;
mod net;
mod notification;

use std::{error::Error, process::ExitCode, thread, time::Duration};

use cli::Cli;
use notification::Notification;

const BATTERY_LIFETIME_HOURS: u32 = 6;
const DELAY: Duration = Duration::from_secs(42);

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
    Cli::Reboot => {
      api::reboot(&api::login()?)?;
      Notification::new("Rebooting ..").show()?;
    }
    Cli::Off => {
      api::off(&api::login()?)?;
      Notification::new("Switching off ..").show()?;
    }
  }
  Ok(())
}

fn info() -> Result<()> {
  let Ok(ref auth_cookie) = api::login() else {
    let battery_status =
      last_battery_short_status().unwrap_or_default();
    Notification::new(format!("Disconnected {battery_status}"))
      .show()?;
    return Ok(());
  };
  let battery = api::battery(auth_cookie)?;
  let net = api::net(auth_cookie)?;
  Notification::new(format!("{battery}\t\t{net}"))
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
