use crate::util::freeotp::OtpItem;

pub mod dmenu;

pub trait Ui {
  fn choose_token(list: Vec<OtpItem>) -> Option<OtpItem>;
}
