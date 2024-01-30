use std::fmt;

pub(crate) enum Net {
  Lte(u8),
  Gsm(u8),
  NoSignal,
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
