use oteepee::{ui, util::freeotp};

fn main() {
  let json_str =
    std::fs::read_to_string("./example.test.json").expect("Something went wrong reading the file");
  let serialized = freeotp::OtpJson::parse(&json_str).unwrap();

  oteepee::run::<ui::dmenu::DMenu>(serialized)
}
