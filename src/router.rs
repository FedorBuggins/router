use crate::{
  api::{self, AuthCookie},
  battery::Battery,
  net::Net,
  Result,
};

#[derive(Clone, Copy)]
pub(crate) enum Command {
  Update,
  Off,
  Reboot,
  OffWhenCharged(bool),
}

#[derive(Default, Clone, Copy)]
pub(crate) enum Status {
  #[default]
  Disconnected,
  On,
  ShuttingDown,
  Rebooting,
  Off,
}

impl Status {
  pub(crate) fn is_on(self) -> bool {
    matches!(self, Self::On)
  }

  pub(crate) fn as_str(&self) -> &str {
    match self {
      Status::Disconnected => "disconnected",
      Status::On => "on",
      Status::ShuttingDown => "shutting down ..",
      Status::Rebooting => "rebooting ..",
      Status::Off => "off",
    }
  }
}

#[derive(Default)]
pub(crate) struct Router {
  pub(crate) status: Status,
  pub(crate) battery: Option<Battery>,
  pub(crate) net: Option<Net>,
  pub(crate) off_when_charged: bool,
}

impl Router {
  pub(crate) fn handle(&mut self, cmd: Command) -> Result<()> {
    match cmd {
      Command::Update => {
        if self.update().is_err() {
          self.handle_connection_error();
        }
      }
      Command::Off if self.status.is_on() => {
        self.off(&api::login()?)?;
      }
      Command::Reboot if self.status.is_on() => {
        self.status = Status::Rebooting;
        api::reboot(&api::login()?)?;
      }
      Command::OffWhenCharged(off_when_charged) => {
        self.off_when_charged = off_when_charged;
      }
      _ => (),
    }
    Ok(())
  }

  fn update(&mut self) -> Result<()> {
    let auth_cookie = &api::login()?;
    let battery = api::battery(auth_cookie)?;
    let net = api::net(auth_cookie)?;
    let should_off = self.should_off(&battery);
    self.status = Status::On;
    self.battery = Some(battery);
    self.net = Some(net);
    if should_off {
      self.off(auth_cookie)?;
    }
    Ok(())
  }

  fn should_off(&mut self, new_battery: &Battery) -> bool {
    self.off_when_charged
      && (new_battery.charged_enough()
        || self.charging_off(new_battery))
  }

  fn charging_off(&mut self, new_battery: &Battery) -> bool {
    self.charging() && !new_battery.charging
  }

  pub(crate) fn charging(&self) -> bool {
    self.battery.as_ref().is_some_and(|b| b.charging)
  }

  pub(crate) fn charged_enough(&self) -> bool {
    self.battery.as_ref().is_some_and(Battery::charged_enough)
  }

  fn off(&mut self, auth_cookie: &AuthCookie) -> Result<()> {
    self.status = Status::ShuttingDown;
    self.off_when_charged = false;
    api::off(auth_cookie)?;
    Ok(())
  }

  fn handle_connection_error(&mut self) {
    self.status = match self.status {
      Status::On => Status::Disconnected,
      Status::ShuttingDown => Status::Off,
      other => other,
    };
    self.net = None;
  }
}
