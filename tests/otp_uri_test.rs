#[cfg(test)]
mod test {
  use oteepee::util::{hashing::HashingAlgorithm, uri::OtpUri};

  #[test]
  fn totp_parsing() {
    let test_totp_uri =
      "otpauth://totp/Fancy%20Test?secret=JBSWY3DPEHPK3PXP&algorithm=SHA1&digits=9&period=60";

    let expected = OtpUri {
      name: "/Fancy%20Test".to_string(),
      secret: vec![72, 101, 108, 108, 111, 33, 222, 173, 190, 239],
      digits: 9,
      period: 60,
      algorithm: HashingAlgorithm::SHA1,
    };

    assert_eq!(OtpUri::parse(test_totp_uri), Some(expected));
  }
}
