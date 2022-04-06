pub fn get_current_time() -> u64 {
  let now = std::time::SystemTime::now()
    .duration_since(std::time::UNIX_EPOCH)
    .expect("Time differnece error");

  now.as_secs()
}

pub fn get_otp(secret: &[u8], digits: u32, period: u64, time: u64) -> u32 {
  crate::otp::hotp::get_otp(secret, digits, time / period)
}
