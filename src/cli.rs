use std::{env, process};

const HELP_TXT: &str = include_str!("../help.txt");

pub(crate) enum Cli {
  Info,
  Watch,
  Charge,
  Reboot,
  Off,
}

impl Cli {
  pub(crate) fn parse() -> Self {
    let args = &mut env::args();
    match args.nth(1).unwrap_or("info".into()).as_str() {
      "info" => Self::Info,
      "watch" => Self::Watch,
      "charge" => Self::Charge,
      "reboot" => Self::Reboot,
      "off" => Self::Off,
      "help" | "--help" | "-h" => {
        print!("{HELP_TXT}");
        process::exit(0)
      }
      _ => {
        eprint!("Unknown command\n\n{HELP_TXT}");
        process::exit(1)
      }
    }
  }
}
