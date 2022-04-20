use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use oteepee::{
  otp::totp,
  util::{
    hashing::HashingAlgorithm,
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
  pub secret: Vec<i32>,

  #[serde(rename = "type")]
  pub otp_type: OtpType,
}

#[derive(Serialize, Deserialize, Debug)]
struct OtpJson {
  #[serde(rename = "tokenOrder")]
  token_order: Vec<String>,
  tokens: Vec<OtpItem>,
}

fn main() {
  // let args: Vec<String> = std::env::args().collect();
  // let uri = parse_otp_uri(&args[1]).expect("Unable to parse otp uri");
  // println!("{:?}", uri);

  // let otp = totp::get_otp(
  //   &uri.secret,
  //   uri.digits,
  //   uri.period,
  //   totp::get_current_time(),
  // );
  // println!("{}: {}", uri.name, otp);

  let json_str =
    std::fs::read_to_string("./example.json").expect("Something went wrong reading the file");

  let serialized: OtpJson = serde_json::from_str(&json_str).unwrap();

  println!("{:?}", serialized)
}
