mod api;
mod battery;
mod cli;
mod net;
mod router;

use std::{error::Error, sync::mpsc, thread, time::Duration};

use termux_notification::TermuxNotification;

use crate::{
  cli::Cli,
  router::{Command, Router},
};

const BIN_NAME: &str = env!("CARGO_BIN_NAME");
const BATTERY_LIFETIME: Duration = Duration::from_secs(7 * 60 * 60);
const TICK: Duration = Duration::from_secs(13);

type Result<T, E = Box<dyn Error>> = std::result::Result<T, E>;

fn main() -> Result<()> {
  match Cli::parse() {
    Cli::Watch => watch(),
    Cli::Reboot => api::reboot(&api::login()?),
    Cli::Off => api::off(&api::login()?),
  }
  .map_err(|err| {
    let _ = show_error_notification(&format!("{err}\n\n{err:?}"));
    err
  })
}

fn watch() -> Result<()> {
  termux_notification::callbacks::init_socket();
  let router = &mut Router::default();
  let (tx, rx) = mpsc::channel();
  spawn_ticks_loop(tx.clone());
  loop {
    let cmd = rx.recv()?;
    router.handle(cmd)?;
    show_notification(router, &tx)?;
  }
}

fn spawn_ticks_loop(tx: mpsc::Sender<Command>) {
  thread::spawn(move || loop {
    tx.send(Command::Update).unwrap();
    thread::sleep(TICK);
  });
}

fn show_notification(
  router: &Router,
  tx: &mpsc::Sender<Command>,
) -> Result<()> {
  let status = router.status.as_str();
  let off_when_charged_badge = off_when_charged_badge(router);
  let mut noti = TermuxNotification::new();
  noti
    .id(BIN_NAME)
    .title(format!("Router > {status} {off_when_charged_badge}"))
    .content(info(router))
    .ongoing(true)
    .alert_once(true)
    .icon("router");
  if router.status.is_on() {
    set_buttons(&mut noti, router, tx);
  }
  noti.show()?;
  Ok(())
}

fn off_when_charged_badge(router: &Router) -> &str {
  if router.off_when_charged && router.charging() {
    "🔌"
  } else {
    ""
  }
}

fn info(router: &Router) -> String {
  match (&router.battery, &router.net) {
    (Some(battery), Some(net)) => format!("{battery}\t\t{net}"),
    (Some(battery), None) => format!("{battery}"),
    _ => String::new(),
  }
}

fn set_buttons(
  noti: &mut TermuxNotification,
  router: &Router,
  tx: &mpsc::Sender<Command>,
) {
  let cb = |cmd| {
    let tx = tx.clone();
    move || tx.send(cmd).unwrap()
  };
  noti
    .button1_fn("OFF", cb(Command::Off))
    .button2_fn("REBOOT", cb(Command::Reboot));
  if router.charging()
    && router.battery.as_ref().is_some_and(|b| b.capacity < 90)
  {
    let f = cb(Command::OffWhenCharged(!router.off_when_charged));
    let label = if router.off_when_charged {
      "KEEP ON"
    } else {
      "CHARGE & OFF"
    };
    noti.button3_fn(label, f);
  }
}

fn show_error_notification(content: &str) -> Result<()> {
  TermuxNotification::new()
    .title(format!("{BIN_NAME} Error"))
    .content(content)
    .icon("error")
    .show()?;
  Ok(())
}
