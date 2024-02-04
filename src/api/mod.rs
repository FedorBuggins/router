use std::{ops::Deref, process::Command};

use crate::{battery::Battery, net::Net, Result};

const PARSE_ERROR: &str = "Parse error";

pub(crate) struct AuthCookie(String);

impl Deref for AuthCookie {
  type Target = String;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

pub(crate) fn login() -> Result<AuthCookie> {
  let s = sh(include_str!("login.sh"))?;
  let auth_cookie =
    slice_between(&s, "Set-cookie: ", ";").ok_or(PARSE_ERROR)?;
  Ok(AuthCookie(auth_cookie.to_string()))
}

pub(crate) fn net(auth_cookie: &AuthCookie) -> Result<Net> {
  const GET_NET_INFO_SH: &str = include_str!("get_net_info.sh");
  let s = sh(&GET_NET_INFO_SH.replace("{auth_cookie}", auth_cookie))?;
  let level = xml_field(&s, "rssi")?.parse::<u8>()?;
  let net = match xml_field(&s, "sys_mode")?.parse::<u8>()? {
    2 | 3 => Net::Lte(level),
    1 => Net::Gsm(level),
    _ => Net::NoSignal,
  };
  Ok(net)
}

pub(crate) fn battery(auth_cookie: &AuthCookie) -> Result<Battery> {
  const GET_BATTERY_INFO_SH: &str =
    include_str!("get_battery_info.sh");
  let s =
    sh(&GET_BATTERY_INFO_SH.replace("{auth_cookie}", auth_cookie))?;
  Ok(Battery {
    capacity: xml_field(&s, "capacity")?.parse::<u8>()?,
    voltage: xml_field(&s, "voltage_now")?.parse::<f32>()? / 1000.,
    charging: xml_field(&s, "usbchg_status")?.parse::<u8>()? == 1,
  })
}

pub(crate) fn reboot(auth_cookie: &AuthCookie) -> Result<()> {
  const REBOOT_SH: &str = include_str!("reboot.sh");
  sh(&REBOOT_SH.replace("{auth_cookie}", auth_cookie))?;
  Ok(())
}

pub(crate) fn off(auth_cookie: &AuthCookie) -> Result<()> {
  const POWER_OFF_SH: &str = include_str!("power_off.sh");
  sh(&POWER_OFF_SH.replace("{auth_cookie}", auth_cookie))?;
  Ok(())
}

fn sh(script: &str) -> Result<String> {
  let output = Command::new("sh").args(["-c", script]).output()?;
  Ok(String::from_utf8(output.stdout)?)
}

fn slice_between<'a>(
  s: &'a str,
  from: &str,
  to: &str,
) -> Option<&'a str> {
  let start = s.find(from)? + from.len();
  let end = start + s[start..].find(to)?;
  Some(&s[start..end])
}

fn xml_field<'a>(s: &'a str, field: &str) -> Result<&'a str> {
  slice_between(s, &format!("<{field}>"), &format!("</{field}>"))
    .ok_or(PARSE_ERROR.into())
}
