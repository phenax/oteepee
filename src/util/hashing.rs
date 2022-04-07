#[derive(Debug)]
pub enum HashingAlgorithm {
  SHA1,
  SHA256,
  SHA512,
}

impl HashingAlgorithm {
  pub fn from_string(s: &str) -> Option<HashingAlgorithm> {
    match s {
      "SHA1" => Some(HashingAlgorithm::SHA1),
      "SHA256" => Some(HashingAlgorithm::SHA256),
      "SHA512" => Some(HashingAlgorithm::SHA512),
      _ => None,
    }
  }
}
