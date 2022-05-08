#[derive(Debug, Clone, PartialEq)]
pub struct OtpSecret(pub Vec<u8>);

impl serde::Serialize for OtpSecret {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    serializer.collect_seq(self.0.iter().map(|&m| (m as i32 - 256) as i8))
  }
}

impl<'de> serde::Deserialize<'de> for OtpSecret {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    Vec::<i32>::deserialize(deserializer).map(|secret| {
      let sec = secret
        .iter()
        .map(|&i| u8::try_from((i + 256) % 256).expect("!secret"))
        .collect::<Vec<u8>>();
      OtpSecret(sec)
    })
  }
}
