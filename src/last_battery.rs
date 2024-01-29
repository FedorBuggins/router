use std::sync::Mutex;

use super::battery::Battery;

static LAST_BATTERY: Mutex<Option<Battery>> = Mutex::new(None);

pub(crate) fn get() -> Option<Battery> {
  LAST_BATTERY.lock().ok()?.clone()
}

pub(crate) fn set(battery: Battery) {
  *LAST_BATTERY.lock().unwrap_or_else(|_| {
    panic!("Can't lock LAST_BATTERY: {LAST_BATTERY:?}")
  }) = Some(battery);
}
