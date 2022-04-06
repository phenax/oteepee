pub fn get_otp(secret: &[u8], digits: u32, period: u64, time: u64) -> u32 {
  crate::otp::hotp::get_otp(secret, digits, time / period)
}
