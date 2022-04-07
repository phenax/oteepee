use std::collections::HashMap;

use oteepee::otp::totp;
use url::Url;

fn decode_secret(data: &str) -> Option<Vec<u8>> {
  base32::decode(base32::Alphabet::RFC4648 { padding: false }, data)
}

#[derive(Debug)]
enum HashingAlgorithm {
  SHA1,
  SHA256,
  SHA512,
}

impl HashingAlgorithm {
  fn from_string(s: &str) -> Option<HashingAlgorithm> {
    match s {
      "SHA1" => Some(HashingAlgorithm::SHA1),
      "SHA256" => Some(HashingAlgorithm::SHA256),
      "SHA512" => Some(HashingAlgorithm::SHA512),
      _ => None,
    }
  }
}

#[derive(Debug)]
struct OtpUri {
  name: String,
  secret: Vec<u8>,
  digits: u32,
  period: u64,
  algorithm: HashingAlgorithm,
}

fn parse_otp_uri(uri: &str) -> Option<OtpUri> {
  let parsed = Url::parse(uri).expect("Unable to parse otp uri");
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

    secret
      .zip(digits)
      .zip(period)
      .zip(algorithm)
      .map(|(((secret, digits), period), algorithm)| OtpUri {
        name: name.to_string(),
        secret,
        digits,
        period,
        algorithm,
      })
  }
}

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
