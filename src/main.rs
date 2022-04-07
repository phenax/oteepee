use oteepee::{otp::totp, util::uri::parse_otp_uri};

fn main() {
  let args: Vec<String> = std::env::args().collect();
  let uri = parse_otp_uri(&args[1]).expect("Unable to parse otp uri");
  println!("{:?}", uri);

  let otp = totp::get_otp(
    &uri.secret,
    uri.digits,
    uri.period,
    totp::get_current_time(),
  );
  println!("{}: {}", uri.name, otp);
}
