use std::{
  env,
  error::Error,
  fmt,
  process::{self, Command as Cmd, ExitCode, Stdio},
  sync::{Mutex, OnceLock},
  thread,
  time::Duration,
};

const BATTERY_LIFETIME_HOURS: u32 = 7;

const HOUR: Duration = Duration::from_secs(3600);
const PARSE_ERROR: &str = "Parse error";
const HELP_TXT: &str = include_str!("../help.txt");

static LAST_BATTERY_CAPACITY: Mutex<Option<u8>> = Mutex::new(None);
static REBOOT_ON_GSM: OnceLock<bool> = OnceLock::new();

type Result<T> = std::result::Result<T, Box<dyn Error>>;

enum Command {
  Info,
  Watch,
  Reboot,
  Off,
}

impl Command {
  fn parse() -> Self {
    let args = &mut env::args();
    match args.nth(1).unwrap_or("info".into()).as_str() {
      "info" => {
        init_reboot_on_gsm_option(args);
        Self::Info
      }
      "watch" => {
        init_reboot_on_gsm_option(args);
        Self::Watch
      }
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
}

fn init_reboot_on_gsm_option(args: &mut env::Args) {
  let reboot_on_gsm =
    args.next().is_some_and(|a| a == "--reboot-on-gsm");
  REBOOT_ON_GSM
    .set(reboot_on_gsm && args.next().as_deref() != Some("false"))
    .expect("Can't set REBOOT_ON_GSM: {REBOOT_ON_GSM:?}");
}

fn main() -> ExitCode {
  match launch() {
    Ok(()) => ExitCode::SUCCESS,
    Err(err) => {
      eprintln!("{err}");
      ExitCode::FAILURE
    }
  }
}

fn launch() -> Result<()> {
  match Command::parse() {
    Command::Info => info(),
    Command::Watch => watch(),
    Command::Reboot => reboot(&login()?),
    Command::Off => off(&login()?),
  }
}

fn watch() -> Result<()> {
  loop {
    info()?;
    thread::sleep(Duration::from_secs(42));
  }
}

fn info() -> Result<()> {
  if show_full_info().is_err() {
    let battery_info = last_battery_info().unwrap_or_default();
    show_status(&format!("Disconnected {battery_info}"))?;
  }
  Ok(())
}

fn last_battery_info() -> Option<String> {
  let capacity = *LAST_BATTERY_CAPACITY.lock().ok()?.as_ref()?;
  let badge = battery_badge(false, capacity);
  Some(format!("{badge} ~{capacity}"))
}

fn show_full_info() -> Result<()> {
  let auth_cookie = &login()?;
  let net = net(auth_cookie)?;
  if reboot_on_gsm() && net.is_gsm() {
    reboot(auth_cookie)
  } else {
    let battery = battery(auth_cookie)?;
    let content = format!("{battery}\t\t{net}");
    show_status_with_controls(&content)
  }
}

fn reboot_on_gsm() -> bool {
  *REBOOT_ON_GSM.get().unwrap_or(&false)
}

fn battery(auth_cookie: &str) -> Result<String> {
  const GET_BATTERY_INFO_SH: &str =
    include_str!("../get_battery_info.sh");
  let s =
    sh(&GET_BATTERY_INFO_SH.replace("{auth_cookie}", auth_cookie))?;
  let capacity = xml_field(&s, "capacity")?.parse::<u8>()?;
  let voltage = xml_field(&s, "voltage_now")?.parse::<f32>()? / 1000.;
  let charging = xml_field(&s, "usbchg_status")?.parse::<u8>()? == 1;
  let badge = battery_badge(charging, capacity);
  let lifetime = to_time_string(lifetime(capacity));
  *LAST_BATTERY_CAPACITY.lock()? = Some(capacity);
  Ok(format!("{badge} {capacity}% {voltage}V ~{lifetime}"))
}

fn battery_badge(charging: bool, capacity: u8) -> &'static str {
  match () {
    () if charging => "⚡️",
    () if capacity > 20 => "🔋",
    () => "🪫",
  }
}

fn lifetime(capacity: u8) -> Duration {
  BATTERY_LIFETIME_HOURS * HOUR * capacity.into() / 100
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

fn reboot(auth_cookie: &str) -> Result<()> {
  const REBOOT_SH: &str = include_str!("../reboot.sh");
  sh(&REBOOT_SH.replace("{auth_cookie}", auth_cookie))?;
  show_status("Rebooting ..")?;
  Ok(())
}

fn off(auth_cookie: &str) -> Result<()> {
  const POWER_OFF_SH: &str = include_str!("../power_off.sh");
  sh(&POWER_OFF_SH.replace("{auth_cookie}", auth_cookie))?;
  show_status("Switching off ..")?;
  Ok(())
}

enum Net {
  Lte(u8),
  Gsm(u8),
  NoSignal,
}

impl Net {
  fn is_gsm(&self) -> bool {
    matches!(self, Self::Gsm(..))
  }
}

impl fmt::Display for Net {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match *self {
      Self::Lte(level) if level > 39 => {
        write!(f, "📡 LTE llll {level}%")
      }
      Self::Lte(level) if level > 27 => {
        write!(f, "📡 LTE lll. {level}%")
      }
      Self::Lte(level) if level > 18 => {
        write!(f, "📡 LTE ll.. {level}%")
      }
      Self::Lte(level) if level > 9 => {
        write!(f, "📡 LTE l... {level}%")
      }
      Self::Lte(level) => {
        write!(f, "📡 LTE .... {level}%")
      }
      Self::Gsm(level) if level > 18 => {
        write!(f, "📠 GSM llll {level}%")
      }
      Self::Gsm(level) if level > 13 => {
        write!(f, "📠 GSM lll. {level}%")
      }
      Self::Gsm(level) if level > 8 => {
        write!(f, "📠 GSM ll.. {level}%")
      }
      Self::Gsm(level) if level > 2 => {
        write!(f, "📠 GSM l... {level}%")
      }
      Self::Gsm(level) => {
        write!(f, "📠 GSM .... {level}%")
      }
      Self::NoSignal => {
        write!(f, "📵 No signal")
      }
    }
  }
}

fn net(auth_cookie: &str) -> Result<Net> {
  const GET_NET_INFO_SH: &str = include_str!("../get_net_info.sh");
  let s = sh(&GET_NET_INFO_SH.replace("{auth_cookie}", auth_cookie))?;
  let level = xml_field(&s, "rssi")?.parse::<u8>()?;
  let net = match xml_field(&s, "sys_mode")?.parse::<u8>()? {
    2 | 3 => Net::Lte(level),
    1 => Net::Gsm(level),
    _ => Net::NoSignal,
  };
  Ok(net)
}

fn xml_field<'a>(s: &'a str, field: &str) -> Result<&'a str> {
  slice_between(s, &format!("<{field}>"), &format!("</{field}>"))
    .ok_or(PARSE_ERROR.into())
}

fn show_status(content: &str) -> Result<()> {
  notify(content).status()?;
  Ok(())
}

fn show_status_with_controls(content: &str) -> Result<()> {
  notify(content)
    .args(["--button1", "OFF"])
    .args(["--button1-action", "~/.cargo/bin/dark-droid off"])
    .args(["--button2", "REBOOT"])
    .args(["--button2-action", "~/.cargo/bin/dark-droid reboot"])
    .status()?;
  Ok(())
}

fn notify(content: &str) -> Cmd {
  let mut cmd = Cmd::new("termux-notification");
  let dark_droid_bin = "~/.cargo/bin/dark-droid";
  let reboot_on_gsm = reboot_on_gsm();
  let badges = if reboot_on_gsm { "🩻🛜" } else { "🛜" };
  let on_tap =
    format!("{dark_droid_bin} info --reboot-on-gsm {reboot_on_gsm}");
  cmd
    .args(["-t", &format!("DarkDroid {badges}")])
    .args(["-c", content])
    .args(["--id", "dark-droid"])
    .arg("--alert-once")
    .arg("--ongoing")
    .args(["--priority", "min"])
    .args(["--icon", "router"])
    .args(["--action", &on_tap]);
  cmd
}

fn login() -> Result<String> {
  let res = sh(include_str!("../login.sh"))?;
  let auth_cookie =
    slice_between(&res, "Set-cookie: ", ";").ok_or(PARSE_ERROR)?;
  Ok(auth_cookie.to_string())
}

fn sh(script: &str) -> Result<String> {
  let output = Cmd::new("sh")
    .arg("-c")
    .arg(script)
    .stderr(Stdio::null())
    .output()?;
  Ok(String::from_utf8(output.stdout)?)
}

fn slice_between<'a>(
  s: &'a str,
  from: &str,
  to: &str,
) -> Option<&'a str> {
  let start = s.find(from)? + from.len();
  let end = start + s[start..].find(to)?;
  s.get(start..end)
}

#[cfg(test)]
mod tests {
  #[test]
  fn bool_to_string() {
    assert_eq!("false/true", format!("{}/{}", false, true));
  }
}
