pub fn decode_secret(data: &str) -> Option<Vec<u8>> {
  base32::decode(base32::Alphabet::RFC4648 { padding: false }, data)
}
