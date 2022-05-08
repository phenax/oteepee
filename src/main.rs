use oteepee::{
  otp::totp,
  util::{self, freeotp},
  util::{dmenu::DmenuItem, hashing::HashingAlgorithm},
};

fn main() -> std::io::Result<()> {
  let json_str =
    std::fs::read_to_string("./example.test.json").expect("Something went wrong reading the file");

  let serialized = freeotp::OtpJson::parse(&json_str).unwrap();

  let selected = util::dmenu::dmenu(
    serialized
      .tokens
      .iter()
      .map(|t| DmenuItem(format!("{}/{}", t.issuer_ext, t.label), t))
      .collect(),
  )?;

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

  Ok(())
}
