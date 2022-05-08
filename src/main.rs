use oteepee::{otp::totp, util::freeotp};

fn main() {
  let json_str =
    std::fs::read_to_string("./example.test.json").expect("Something went wrong reading the file");

  let serialized = freeotp::OtpJson::parse(&json_str).unwrap();

  let item = &serialized.tokens[0];
  // let uri = OtpUri::parse(&args[1]).expect("Unable to parse otp uri");
  // println!("{:?}", uri);

  let otp = totp::get_otp(
    &item.secret.0,
    item.digits.unwrap_or(6),
    item.period.unwrap_or(30),
    totp::get_current_time(),
  );
  println!("{}/{}: {}", item.issuer_ext, item.label, otp);
}
