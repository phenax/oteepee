#[cfg(test)]
mod test {
  use oteepee::{
    otp::totp,
    util::{freeotp::*, hashing::HashingAlgorithm, secret::OtpSecret},
  };

  const FREEOTP_JSON_STR: &str = r#"
    {
      "tokens": [
        {
          "algo": "SHA1",
          "counter": 0,
          "digits": 6,
          "issuerExt": "Segment",
          "label": "akshay@colabra.app",
          "period": 30,
          "secret": [ -184, -155, -148, -148, -145, -223, -34,  -83,  -66, -17 ],
          "type": "TOTP"
        }
      ]
    }
  "#;

  #[test]
  fn freeotp_format_parsing() {
    let freeotp_json = OtpJson::parse(FREEOTP_JSON_STR).expect("!parse");

    assert_eq!(
      freeotp_json,
      OtpJson {
        tokens: vec![OtpItem {
          issuer_ext: "Segment".to_string(),
          label: "akshay@colabra.app".to_string(),
          algo: Some(HashingAlgorithm::SHA1),
          counter: Some(0),
          digits: Some(6),
          period: Some(30),
          secret: OtpSecret(vec![72, 101, 108, 108, 111, 33, 222, 173, 190, 239]),
          otp_type: OtpType::TOTP
        }]
      }
    );
  }

  #[test]
  fn freeotp_format_parsing_eval() {
    let freeotp_json = OtpJson::parse(FREEOTP_JSON_STR).expect("!parse");

    let token = &freeotp_json.tokens[0];

    let secret = &token.secret.0;
    let digits = token.digits.expect("!digits");
    let period = token.period.expect("!period");

    let time = 1651991203;

    assert_eq!(totp::get_otp(secret, digits, period, time), 982647);
  }
}
