use std::collections::HashMap;

use super::base32::decode_secret;
use super::hashing::HashingAlgorithm;

#[derive(Debug)]
pub struct OtpUri {
  pub name: String,
  pub secret: Vec<u8>,
  pub digits: u32,
  pub period: u64,
  pub algorithm: HashingAlgorithm,
}

pub fn parse_otp_uri(uri: &str) -> Option<OtpUri> {
  let parsed = url::Url::parse(uri).ok();
  if let Some(parsed) = parsed {
    if parsed.scheme() != "otpauth" {
      None
    } else {
      let query_params = parsed.query_pairs().collect::<HashMap<_, _>>();
      let name = parsed.path(); // TODO: Cleanup url encoded name
      let secret = query_params.get("secret").and_then(|s| decode_secret(s));
      let digits = query_params
        .get("digits")
        .and_then(|s| s.parse::<u32>().ok());
      let period = query_params
        .get("period")
        .and_then(|s| s.parse::<u64>().ok());
      let algorithm = query_params
        .get("algorithm")
        .and_then(|s| HashingAlgorithm::from_string(s));

      secret.zip(digits).zip(period).zip(algorithm).map(
        |(((secret, digits), period), algorithm)| OtpUri {
          name: name.to_string(),
          secret,
          digits,
          period,
          algorithm,
        },
      )
    }
  } else {
    None
  }
}
