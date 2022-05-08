mod menu;

use super::Ui;
use crate::util::freeotp::OtpItem;

pub struct Tui;

impl Ui for Tui {
  fn choose_token(list: Vec<OtpItem>) -> Option<OtpItem> {
    menu::open_menu().expect("fuck!");
    None
    // Self::tui(
    //   list
    //     .iter()
    //     .map(|t| (format!("{}/{}", t.issuer_ext, t.label), t))
    //     .collect(),
    // )
    // .expect("!dmenu")
    // .cloned()
  }
}
