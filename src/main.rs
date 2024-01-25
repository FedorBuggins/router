#![warn(clippy::pedantic)]

use std::{
  env,
  error::Error,
  process::{Command, Stdio},
  thread,
  time::Duration,
};

const PARSE_ERROR: &str = "Parse error";

fn main() -> Result<(), Box<dyn Error>> {
  match env::args().nth(1).unwrap_or_default().as_str() {
    "off" | "--off" => power_off(&login()?),
    "reboot" | "--reboot" => reboot(&login()?),
    _ => loop {
      if show_info().is_err() {
        show_status("Disconnected")?;
      }
      thread::sleep(Duration::from_secs(40));
    },
  }
}

fn show_info() -> Result<(), Box<dyn Error>> {
  let auth_cookie = &login()?;
  let content =
    battery_info(auth_cookie)? + " " + &net_info(auth_cookie)?;
  show_status_with_controls(&content)?;
  Ok(())
}

fn battery_info(auth_cookie: &str) -> Result<String, Box<dyn Error>> {
  const GET_BATTERY_INFO_SH: &str =
    include_str!("../get_battery_info.sh");
  let s =
    sh(&GET_BATTERY_INFO_SH.replace("{auth_cookie}", auth_cookie))?;
  let c = xml_field(&s, "capacity")
    .ok_or(PARSE_ERROR)?
    .parse::<u8>()?;
  let v = xml_field(&s, "voltage_now").ok_or(PARSE_ERROR)?;
  let ch = match xml_field(&s, "usbchg_status")
    .ok_or(PARSE_ERROR)?
    .parse::<u8>()?
  {
    1 => "⚡️",
    _ if c > 20 => "🔋",
    _ => "🪫",
  };
  Ok(format!("{ch}{c}% {v} mV"))
}

fn reboot(auth_cookie: &str) -> Result<(), Box<dyn Error>> {
  const REBOOT_SH: &str = include_str!("../reboot.sh");
  sh(&REBOOT_SH.replace("{auth_cookie}", auth_cookie))?;
  show_status("Rebooting ..")?;
  Ok(())
}

fn power_off(auth_cookie: &str) -> Result<(), Box<dyn Error>> {
  const POWER_OFF_SH: &str = include_str!("../power_off.sh");
  sh(&POWER_OFF_SH.replace("{auth_cookie}", auth_cookie))?;
  show_status("Switching off ..")?;
  Ok(())
}

fn net_info(auth_cookie: &str) -> Result<String, Box<dyn Error>> {
  const GET_NET_INFO_SH: &str = include_str!("../get_net_info.sh");
  let s = sh(&GET_NET_INFO_SH.replace("{auth_cookie}", auth_cookie))?;
  let m = xml_field(&s, "sys_mode")
    .ok_or(PARSE_ERROR)?
    .parse::<u8>()?;
  let r = xml_field(&s, "rssi").ok_or(PARSE_ERROR)?.parse::<u8>()?;
  let s = match m {
    1 if r > 18 => format!("📡 GSM llll {r}%"),
    1 if r > 13 => format!("📡 GSM lll. {r}%"),
    1 if r > 8 => format!("📡 GSM ll.. {r}%"),
    1 if r > 2 => format!("📡 GSM l... {r}%"),
    1 => format!("📡 GSM .... {r}%"),
    2 | 3 if r > 39 => format!("📡 LTE llll {r}%"),
    2 | 3 if r > 27 => format!("📡 LTE lll. {r}%"),
    2 | 3 if r > 18 => format!("📡 LTE ll.. {r}%"),
    2 | 3 if r > 9 => format!("📡 LTE l... {r}%"),
    2 | 3 => format!("📡 LTE .... {r}%"),
    _ => "📵 No signal".to_string(),
  };
  Ok(s)
}

fn xml_field(s: &str, field: &str) -> Option<String> {
  extract(s, &format!("<{field}>"), &format!("</{field}>"))
}

fn show_status(content: &str) -> Result<(), Box<dyn Error>> {
  notify(content).status()?;
  Ok(())
}

fn show_status_with_controls(
  content: &str,
) -> Result<(), Box<dyn Error>> {
  notify(content)
    .args(["--button1", "OFF"])
    .args(["--button1-action", "~/.cargo/bin/dark_droid off"])
    .args(["--button2", "REBOOT"])
    .args(["--button2-action", "~/.cargo/bin/dark_droid reboot"])
    .status()?;
  Ok(())
}

fn notify(content: &str) -> Command {
  let mut cmd = Command::new("termux-notification");
  cmd
    .args(["-t", "DarkDroid"])
    .args(["-c", content])
    .args(["--id", "dark_droid"])
    .arg("--alert-once")
    .args(["--priority", "min"])
    .args(["--icon", "router"]);
  cmd
}

fn login() -> Result<String, Box<dyn Error>> {
  let res = sh(include_str!("../login.sh"))?;
  let auth_cookie =
    extract(&res, "Set-cookie: ", ";").ok_or(PARSE_ERROR)?;
  Ok(auth_cookie)
}

fn sh(script: &str) -> Result<String, Box<dyn Error>> {
  let output = Command::new("sh")
    .arg("-c")
    .arg(script)
    .stderr(Stdio::null())
    .output()?;
  Ok(String::from_utf8(output.stdout)?)
}

fn extract(s: &str, from: &str, to: &str) -> Option<String> {
  let start = s.find(from)? + from.len();
  let end = start + s[start..].find(to)?;
  Some(s[start..end].to_string())
}
