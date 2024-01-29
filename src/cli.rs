use std::{env, ops, process};

#[derive(Clone, Copy)]
pub(crate) struct RebootOnGsm(pub(crate) bool);

impl ops::Deref for RebootOnGsm {
  type Target = bool;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

pub(crate) enum Cli {
  Info,
  Watch(RebootOnGsm),
  Reboot,
  Off,
}

impl Cli {
  pub(crate) fn parse() -> Self {
    pub(crate) const HELP_TXT: &str = include_str!("../help.txt");
    let args = &mut env::args();
    match args.nth(1).unwrap_or("info".into()).as_str() {
      "info" => Self::Info,
      "watch" => Self::Watch(RebootOnGsm(should_reboot_on_gsm(args))),
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

fn should_reboot_on_gsm(args: &mut env::Args) -> bool {
  args.next().is_some_and(|a| a == "--reboot-on-gsm")
    && args.next().as_deref() != Some("false")
}
