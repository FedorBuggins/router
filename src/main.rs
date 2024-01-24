#![warn(clippy::pedantic)]

use std::{
  error::Error,
  process::{Command, Stdio},
  thread,
  time::Duration,
};

fn main() -> Result<(), Box<dyn Error>> {
  loop {
    if show_status().is_err() {
      notify("Disconnected")?;
    }
    thread::sleep(Duration::from_secs(40));
  }
}

fn show_status() -> Result<(), Box<dyn Error>> {
  let auth_cookie = &login()?;
  let content =
    battery_info(auth_cookie)? + " " + &net_info(auth_cookie)?;
  notify(&content)?;
  Ok(())
}

fn battery_info(auth_cookie: &str) -> Result<String, Box<dyn Error>> {
  let out = Command::new("sh")
    .arg("-c")
    .arg(
      include_str!("../get_battery_info.sh")
        .replace("{auth_cookie}", auth_cookie),
    )
    .stderr(Stdio::null())
    .output()?;
  let s = String::from_utf8(out.stdout)?;
  // println!("{s}");
  let c = xml_field(&s, "capacity")
    .ok_or("Parse error")?
    .parse::<u8>()?;
  let v = xml_field(&s, "voltage_now").ok_or("Parse error")?;
  let ch = match xml_field(&s, "usbchg_status")
    .ok_or("Parse error")?
    .parse::<u8>()?
  {
    1 => "⚡️",
    _ if c > 20 => "🔋",
    _ => "🪫",
  };
  Ok(format!("{ch}{c}% {v} mV"))
}

fn net_info(auth_cookie: &str) -> Result<String, Box<dyn Error>> {
  let out = Command::new("sh")
    .arg("-c")
    .arg(
      include_str!("../get_net_info.sh")
        .replace("{auth_cookie}", auth_cookie),
    )
    .stderr(Stdio::null())
    .output()?;
  let s = String::from_utf8(out.stdout)?;
  let m = xml_field(&s, "sys_mode")
    .ok_or("Parse error")?
    .parse::<u8>()?;
  let r =
    xml_field(&s, "rssi").ok_or("Parse error")?.parse::<u8>()?;
  // println!("{s}");
  let s = match m {
    1 if r > 18 => "GSM llll",
    1 if r > 13 => "GSM lll.",
    1 if r > 8 => "GSM ll..",
    1 if r > 2 => "GSM l...",
    1 => "GSM ....",
    2 | 3 if r > 39 => "LTE llll",
    2 | 3 if r > 27 => "LTE lll.",
    2 | 3 if r > 18 => "LTE ll..",
    2 | 3 if r > 9 => "LTE l...",
    2 | 3 => "LTE ....",
    _ => "No signal",
  };
  Ok(format!("📡 {s} {r}%"))
}

fn xml_field(s: &str, field: &str) -> Option<String> {
  extract(s, &format!("<{field}>"), &format!("</{field}>"))
}

fn notify(content: &str) -> Result<(), Box<dyn Error>> {
  Command::new("termux-notification")
    .args(["-t", "DarkDroid"])
    .args(["-c", content])
    .args(["--id", "dark_droid"])
    .arg("--alert-once")
    .args(["--priority", "min"])
    .args(["--icon", "router"])
    .status()?;
  Ok(())
}

fn login() -> Result<String, Box<dyn Error>> {
  let output = Command::new("sh")
    .arg("-c")
    .arg(include_str!("../login.sh"))
    .stderr(Stdio::null())
    .output()?;
  let stdout = String::from_utf8(output.stdout)?;
  let auth_cookie =
    extract(&stdout, "Set-cookie: ", ";").ok_or("Parse error")?;
  Ok(auth_cookie)
}

fn extract(s: &str, from: &str, to: &str) -> Option<String> {
  let start = s.find(from)? + from.len();
  let end = start + s[start..].find(to)?;
  Some(s[start..end].to_string())
}
