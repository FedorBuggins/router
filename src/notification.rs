use std::{io, process::Command};

use crate::BIN_NAME;

const BIN: &str = concat!("~/.cargo/bin/", env!("CARGO_BIN_NAME"));

pub(crate) struct Notification(Command);

impl Notification {
  fn new(content: impl Into<String>) -> Self {
    let mut cmd = Command::new("termux-notification");
    cmd
      .args(["--title", "DarkDroid 🛜"])
      .args(["--content", &content.into()])
      .args(["--icon", "router"]);
    Self(cmd)
  }

  pub(crate) fn ongoing(content: impl Into<String>) -> Self {
    let mut noti = Self::new(content);
    let on_tap = &format!("{BIN} info");
    noti
      .0
      .args(["--id", BIN_NAME])
      .arg("--alert-once")
      .arg("--ongoing")
      .args(["--priority", "min"])
      .args(["--action", on_tap]);
    noti
  }

  pub(crate) fn common(content: impl Into<String>) -> Self {
    let mut noti = Self::new(content);
    noti.0.args(["--id", &format!("{BIN_NAME}--common")]);
    noti
  }

  pub(crate) fn set_power_buttons(&mut self) -> &mut Self {
    self
      .0
      .args(["--button1", "OFF"])
      .args(["--button1-action", &format!("{BIN} off")])
      .args(["--button2", "REBOOT"])
      .args(["--button2-action", &format!("{BIN} reboot")]);
    self
  }

  pub(crate) fn on_delete(&mut self, action: &str) -> &mut Self {
    self.0.args(["--on-delete", action]);
    self
  }

  pub(crate) fn show(&mut self) -> io::Result<()> {
    self.0.output()?;
    Ok(())
  }
}
