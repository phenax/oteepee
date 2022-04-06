use hmac::{Hmac, Mac};
use sha1::Sha1;

pub fn hash(secret: &[u8], msg: &[u8]) -> Vec<u8> {
  type HmacSha1 = Hmac<Sha1>;
  let mut hmac = HmacSha1::new_from_slice(secret).expect("Error while hash secret");
  hmac.update(msg);
  hmac.finalize().into_bytes()[..].into()
}

pub fn get_otp(secret: &[u8], digits: u32, count: u64) -> u32 {
  let hash = hash(secret, &count.to_be_bytes());
  let offset = (hash[hash.len() - 1] & 0xf) as usize;
  let code = ((hash[offset] & 0x7f) as u32) << 24
    | (hash[offset + 1] as u32) << 16
    | (hash[offset + 2] as u32) << 8
    | (hash[offset + 3] as u32);

  code % (10u32.pow(digits))
}
