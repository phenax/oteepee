use serde::{Deserialize, Serialize};

use oteepee::{
  otp::totp,
  util::{
    hashing::HashingAlgorithm,
    secret::*,
    uri::{parse_otp_uri, OtpUri},
  },
};

#[derive(Serialize, Deserialize, Debug)]
pub enum OtpType {
  TOTP,
  HOTP,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OtpItem {
  #[serde(rename = "issuerExt")]
  pub issuer_ext: String,
  pub label: String,
  pub algo: Option<HashingAlgorithm>,
  pub counter: Option<u32>,
  pub digits: Option<u32>,
  pub period: Option<u64>,
  pub secret: OtpSecret,

  #[serde(rename = "type")]
  pub otp_type: OtpType,
}

#[derive(Serialize, Deserialize, Debug)]
struct OtpJson {
  tokens: Vec<OtpItem>,
}

fn main() {
  let json_str =
    std::fs::read_to_string("./example.json").expect("Something went wrong reading the file");

  let serialized: OtpJson = serde_json::from_str(&json_str).unwrap();

  let item = &serialized.tokens[0];
  // let uri = parse_otp_uri(&args[1]).expect("Unable to parse otp uri");
  // println!("{:?}", uri);

  let otp = totp::get_otp(
    &item.secret.0,
    item.digits.unwrap_or(6),
    item.period.unwrap_or(30),
    totp::get_current_time(),
  );
  println!("{}/{}: {}", item.issuer_ext, item.label, otp);
}
