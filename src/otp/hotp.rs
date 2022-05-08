use crate::util::hashing::HashingAlgorithm;

pub fn get_otp(secret: &[u8], algo: &HashingAlgorithm, digits: u32, count: u64) -> u32 {
  let hash = algo.hash(secret, &count.to_be_bytes());
  let offset = (hash[hash.len() - 1] & 0xf) as usize;
  let code = ((hash[offset] & 0x7f) as u32) << 24
    | (hash[offset + 1] as u32) << 16
    | (hash[offset + 2] as u32) << 8
    | (hash[offset + 3] as u32);

  code % (10u32.pow(digits))
}
