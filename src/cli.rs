use std::{env, process};

const HELP_TXT: &str = include_str!("../help.txt");

pub(crate) enum Cli {
  Watch,
  Reboot,
  Off,
}

impl Cli {
  pub(crate) fn parse() -> Self {
    let args = &mut env::args();
    match args.nth(1).unwrap_or("watch".into()).as_str() {
      "watch" => Self::Watch,
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
