pub fn decode_secret(data: &str) -> Option<Vec<u8>> {
  base32::decode(base32::Alphabet::RFC4648 { padding: false }, data)
}

fn main() {
  let secret_str = "";
  let digits = 6;
  let period = 30;

  let now = std::time::SystemTime::now()
    .duration_since(std::time::UNIX_EPOCH)
    .expect("Time differnece error");

  let secret = decode_secret(secret_str).expect("Unable to decode secret");

  let otp = oteepee::otp::totp::get_otp(&secret, digits, period, now.as_secs());

  println!("{}", otp);
}
