use crate::{
  otp::totp,
  util::freeotp::{OtpItem, OtpJson},
  util::hashing::HashingAlgorithm,
};

pub mod otp;
pub mod ui;
pub mod util;

pub fn get_item<Ui: ui::Ui>(otp: OtpJson) -> Option<(u32, OtpItem)> {
  let selected = Ui::choose_token(otp.tokens);

  selected.map(|item| {
    let otp = totp::get_otp(
      &item.secret.0,
      item.algo.as_ref().unwrap_or(&HashingAlgorithm::SHA1),
      item.digits.unwrap_or(6),
      item.period.unwrap_or(30),
      totp::get_current_time(),
    );

    (otp, item)
  })
}

pub fn run<Ui: ui::Ui>(otp: OtpJson) {
  let selected = get_item::<Ui>(otp);

  if let Some((otp, item)) = selected {
    println!("{:?} {:?}", otp, item)
  }
}
