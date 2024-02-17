use std::{
  collections::HashSet,
  env, process,
  sync::atomic::{AtomicBool, Ordering},
};

const HELP_TXT: &str = include_str!("../help.txt");

static VERBOSE: AtomicBool = AtomicBool::new(false);

pub(crate) enum Cli {
  Watch,
  Reboot,
  Off,
}

impl Cli {
  pub(crate) fn parse() -> Self {
    let mut args: HashSet<_> = env::args().skip(1).collect();
    extract_verbose_option(&mut args);
    match args.iter().next().map_or("watch", String::as_str) {
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

  pub(crate) fn verbose() -> bool {
    VERBOSE.load(Ordering::Relaxed)
  }
}

fn extract_verbose_option(args: &mut HashSet<String>) {
  VERBOSE.store(
    ["--verbose", "-v"].into_iter().any(|v| args.remove(v)),
    Ordering::Relaxed,
  );
}
