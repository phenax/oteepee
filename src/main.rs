use oteepee::otp::totp;

pub fn decode_secret(data: &str) -> Option<Vec<u8>> {
  base32::decode(base32::Alphabet::RFC4648 { padding: false }, data)
}

fn main() {
  let args: Vec<String> = std::env::args().collect();
  let secret_str = &args[1];
  let digits = 6;
  let period = 30;

  let secret = decode_secret(secret_str).expect("Unable to decode secret");
  let otp = totp::get_otp(&secret, digits, period, totp::get_current_time());

  println!("{}", otp);
}
