use crate::util::{hashing::HashingAlgorithm, secret::*};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum OtpType {
  TOTP,
  HOTP,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct OtpJson {
  pub tokens: Vec<OtpItem>,
}

impl OtpJson {
  pub fn parse(json: &str) -> Result<Self, serde_json::Error> {
    serde_json::from_str(json)
  }
}
