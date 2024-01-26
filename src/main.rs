use std::{
  env,
  error::Error,
  process::{Command, Stdio},
  thread,
  time::Duration,
};

const MAX_LIFETIME_HOURS: u32 = 6;

const HOUR: Duration = Duration::from_secs(3600);
const PARSE_ERROR: &str = "Parse error";

type Any<T = ()> = Result<T, Box<dyn Error>>;

fn main() -> Any {
  match env::args().nth(1).unwrap_or_default().as_str() {
    "info" | "" => info(),
    "watch" => watch(),
    "reboot" => reboot(&login()?),
    "off" => off(&login()?),
    _ => Err("Unknown command")?,
  }
}

fn watch() -> Any {
  loop {
    info()?;
    thread::sleep(Duration::from_secs(42));
  }
}

fn info() -> Any {
  if show_full_info().is_err() {
    show_status("Disconnected")?;
  }
  Ok(())
}

fn show_full_info() -> Any {
  let auth_cookie = &login()?;
  let battery_info = battery_info(auth_cookie)?;
  let net_info = net_info(auth_cookie)?;
  let content = format!("{battery_info}\t\t{net_info}");
  show_status_with_controls(&content)?;
  Ok(())
}

fn battery_info(auth_cookie: &str) -> Any<String> {
  const GET_BATTERY_INFO_SH: &str =
    include_str!("../get_battery_info.sh");
  let s =
    sh(&GET_BATTERY_INFO_SH.replace("{auth_cookie}", auth_cookie))?;
  let capacity = xml_field(&s, "capacity")
    .ok_or(PARSE_ERROR)?
    .parse::<u8>()?;
  let voltage = xml_field(&s, "voltage_now")
    .ok_or(PARSE_ERROR)?
    .parse::<f32>()?
    / 1000.;
  let charging = xml_field(&s, "usbchg_status")
    .ok_or(PARSE_ERROR)?
    .parse::<u8>()?
    == 1;
  let badge = badge(charging, capacity);
  let lifetime = to_time_string(lifetime(capacity));
  Ok(format!("{badge} {capacity}% {voltage}V ~{lifetime}"))
}

fn badge(charging: bool, capacity: u8) -> &'static str {
  match () {
    () if charging => "⚡️",
    () if capacity > 20 => "🔋",
    () => "🪫",
  }
}

fn lifetime(capacity: u8) -> Duration {
  MAX_LIFETIME_HOURS * HOUR * capacity.into() / 100
}

fn to_time_string(dur: Duration) -> String {
  let h = dur.as_secs_f32() / HOUR.as_secs_f32();
  if h > 1. {
    let h = (h * 10.).round() / 10.;
    format!("{h}h")
  } else {
    let m = (h * 60.).round();
    format!("{m}m")
  }
}

fn reboot(auth_cookie: &str) -> Any {
  const REBOOT_SH: &str = include_str!("../reboot.sh");
  sh(&REBOOT_SH.replace("{auth_cookie}", auth_cookie))?;
  show_status("Rebooting ..")?;
  Ok(())
}

fn off(auth_cookie: &str) -> Any {
  const POWER_OFF_SH: &str = include_str!("../power_off.sh");
  sh(&POWER_OFF_SH.replace("{auth_cookie}", auth_cookie))?;
  show_status("Switching off ..")?;
  Ok(())
}

fn net_info(auth_cookie: &str) -> Any<String> {
  const GET_NET_INFO_SH: &str = include_str!("../get_net_info.sh");
  let s = sh(&GET_NET_INFO_SH.replace("{auth_cookie}", auth_cookie))?;
  let m = xml_field(&s, "sys_mode")
    .ok_or(PARSE_ERROR)?
    .parse::<u8>()?;
  let r = xml_field(&s, "rssi").ok_or(PARSE_ERROR)?.parse::<u8>()?;
  let s = match m {
    2 | 3 if r > 39 => format!("📡 LTE llll {r}%"),
    2 | 3 if r > 27 => format!("📡 LTE lll. {r}%"),
    2 | 3 if r > 18 => format!("📡 LTE ll.. {r}%"),
    2 | 3 if r > 9 => format!("📡 LTE l... {r}%"),
    2 | 3 => format!("📡 LTE .... {r}%"),
    1 if r > 18 => format!("📠 GSM llll {r}%"),
    1 if r > 13 => format!("📠 GSM lll. {r}%"),
    1 if r > 8 => format!("📠 GSM ll.. {r}%"),
    1 if r > 2 => format!("📠 GSM l... {r}%"),
    1 => format!("📠 GSM .... {r}%"),
    _ => "📵 No signal".to_string(),
  };
  Ok(s)
}

fn xml_field(s: &str, field: &str) -> Option<String> {
  extract(s, &format!("<{field}>"), &format!("</{field}>"))
}

fn show_status(content: &str) -> Any {
  notify(content).status()?;
  Ok(())
}

fn show_status_with_controls(content: &str) -> Any {
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
    .args(["-t", "DarkDroid 🛜"])
    .args(["-c", content])
    .args(["--id", "dark_droid"])
    .arg("--alert-once")
    .arg("--ongoing")
    .args(["--priority", "min"])
    .args(["--icon", "router"])
    .args(["--action", "~/.cargo/bin/dark_droid info"]);
  cmd
}

fn login() -> Any<String> {
  let res = sh(include_str!("../login.sh"))?;
  let auth_cookie =
    extract(&res, "Set-cookie: ", ";").ok_or(PARSE_ERROR)?;
  Ok(auth_cookie)
}

fn sh(script: &str) -> Any<String> {
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
