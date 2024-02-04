use std::io;

use termux_notification::TermuxNotification;

use crate::BIN_NAME;

const BIN: &str = concat!("~/.cargo/bin/", env!("CARGO_BIN_NAME"));

pub(crate) struct Notification(TermuxNotification);

impl Notification {
  fn new(content: impl Into<String>) -> Self {
    let mut tn = TermuxNotification::new();
    tn.title("DarkDroid 🛜").content(content).icon("router");
    Self(tn)
  }

  pub(crate) fn ongoing(content: impl Into<String>) -> Self {
    let mut noti = Self::new(content);
    noti
      .0
      .id(BIN_NAME)
      .alert_once(true)
      .ongoing(true)
      .on_tap(format!("{BIN} info"));
    noti
  }

  pub(crate) fn common(content: impl Into<String>) -> Self {
    let mut noti = Self::new(content);
    noti.0.id(format!("{BIN_NAME}--common"));
    noti
  }

  pub(crate) fn set_power_buttons(&mut self) -> &mut Self {
    self
      .0
      .button1("OFF", format!("{BIN} off"))
      .button2("REBOOT", format!("{BIN} reboot"));
    self
  }

  pub(crate) fn on_delete(&mut self, action: &str) -> &mut Self {
    self.0.on_delete(action);
    self
  }

  pub(crate) fn show(&self) -> io::Result<()> {
    self.0.show()?;
    Ok(())
  }
}
