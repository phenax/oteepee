pub fn decode_secret(data: &str) -> Option<Vec<u8>> {
  base32::decode(base32::Alphabet::RFC4648 { padding: false }, data)
}

fn main() {
  let secret_str = "";
  let digits = 6;
  let secret = decode_secret(secret_str).expect("Unable to decode secret");
  oteepee::otp::hotp::get_otp(&secret, digits, 0);
}
