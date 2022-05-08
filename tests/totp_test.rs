#[cfg(test)]
mod test {
  use oteepee::{otp::totp, util::hashing::HashingAlgorithm};

  #[test]
  fn totp_otp_generation() {
    let secret = vec![72, 101, 108, 108, 111, 33, 222, 173, 190, 239];
    let algo = HashingAlgorithm::SHA1;
    let digits = 6;
    let period = 30;
    let time = 1651991203;

    assert_eq!(totp::get_otp(&secret, &algo, digits, period, time), 982647);
  }
}
