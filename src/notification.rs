use std::{io, process::Command};

const BIN: &str = concat!("~/.cargo/bin/", env!("CARGO_BIN_NAME"));

pub(crate) struct Notification(Command);

impl Notification {
  pub(crate) fn new(content: impl Into<String>) -> Self {
    let mut cmd = Command::new("termux-notification");
    let on_tap = &format!("{BIN} info");
    cmd
      .args(["--id", env!("CARGO_BIN_NAME")])
      .args(["--title", "DarkDroid 🛜"])
      .args(["--content", &content.into()])
      .arg("--alert-once")
      .arg("--ongoing")
      .args(["--priority", "min"])
      .args(["--icon", "router"])
      .args(["--action", on_tap]);
    Self(cmd)
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

  pub(crate) fn show(&mut self) -> io::Result<()> {
    self.0.output()?;
    Ok(())
  }
}
