use std::{
  collections::HashMap,
  io::Write,
  process::{Command, Stdio},
};

use crate::util::freeotp::OtpItem;

use super::Ui;

pub struct DMenu;

impl Ui for DMenu {
  fn choose_token(list: Vec<OtpItem>) -> Option<OtpItem> {
    Self::dmenu(
      list
        .iter()
        .map(|t| (format!("{}/{}", t.issuer_ext, t.label), t))
        .collect(),
    )
    .expect("!dmenu")
    .cloned()
  }
}

impl DMenu {
  fn dmenu<T: Clone>(lines: Vec<(String, T)>) -> std::io::Result<Option<T>> {
    let mut map: HashMap<String, T> = HashMap::new();
    let mut str_lines = vec![];

    for (key, item) in lines {
      map.insert(key.to_owned(), item);
      str_lines.push(key.trim().to_string());
    }

    let selected = Self::dmenu_raw(str_lines)?;

    Ok(if selected.trim() == "" {
      None
    } else {
      map.get(&selected.trim().to_string()).cloned()
    })
  }

  fn dmenu_raw(lines: Vec<String>) -> std::io::Result<String> {
    let proc = Command::new("dmenu")
      .args(&["-p", "Otp :: "])
      .stdout(Stdio::piped())
      .stdin(Stdio::piped())
      .spawn()?;

    let stdin = proc.stdin.as_ref();
    stdin
      .expect("!stdin")
      .write_all(lines.join("\n").as_bytes())?;

    let output = proc.wait_with_output()?;

    let selected = String::from_utf8(output.stdout).expect("!cant read stdout");
    Ok(selected.trim().to_string())
  }
}
