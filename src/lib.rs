use crate::{otp::totp, util::freeotp::OtpJson, util::hashing::HashingAlgorithm};

pub mod otp;
pub mod ui;
pub mod util;

pub fn run<Ui: ui::Ui>(otp: OtpJson) {
  let selected = Ui::choose_token(otp.tokens);

  if let Some(item) = selected {
    let otp = totp::get_otp(
      &item.secret.0,
      item.algo.as_ref().unwrap_or(&HashingAlgorithm::SHA1),
      item.digits.unwrap_or(6),
      item.period.unwrap_or(30),
      totp::get_current_time(),
    );

    println!("{}/{}: {}", item.issuer_ext, item.label, otp);
  }
}
