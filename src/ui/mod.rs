pub mod dmenu;
pub mod tui;

use crate::util::freeotp::OtpItem;

pub trait Ui {
  fn choose_token(list: Vec<OtpItem>) -> Option<OtpItem>;
}
