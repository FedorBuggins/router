mod api;
mod battery;
mod cli;
mod commands;
mod last_battery;
mod net;
mod notification;

use std::{error::Error, process::ExitCode, thread, time::Duration};

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
    Cli::Info => {
      commands::info()?;
    }
    Cli::Watch => loop {
      commands::info()?;
      thread::sleep(DELAY);
    },
    Cli::Charge => {
      commands::charge()?;
    }
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
