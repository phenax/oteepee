use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha1::Sha1;
use sha2::{Sha256, Sha512};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum HashingAlgorithm {
  SHA1,
  SHA256,
  SHA512,
}

macro_rules! hash {
  ($algo: ident, $secret: expr, $msg: expr) => {{
    let mut hmac = Hmac::<$algo>::new_from_slice($secret).expect("Error while hash secret");
    hmac.update($msg);
    hmac.finalize().into_bytes()[..].into()
  }};
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

  pub fn hash(&self, secret: &[u8], msg: &[u8]) -> Vec<u8> {
    match self {
      Self::SHA1 => hash!(Sha1, secret, msg),
      Self::SHA256 => hash!(Sha256, secret, msg),
      Self::SHA512 => hash!(Sha512, secret, msg),
    }
  }
}
