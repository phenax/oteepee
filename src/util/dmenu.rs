use std::{
  collections::HashMap,
  io::Write,
  process::{Command, Stdio},
};

pub struct DmenuItem<T>(pub String, pub T);

pub fn dmenu<T: Clone>(lines: Vec<DmenuItem<T>>) -> std::io::Result<Option<T>> {
  let mut map: HashMap<String, T> = HashMap::new();
  let mut str_lines = vec![];

  for DmenuItem(key, item) in lines {
    map.insert(key.to_owned(), item);
    str_lines.push(key.trim().to_string());
  }

  let selected = dmenu_raw(str_lines)?;

  Ok(if selected.trim() == "" {
    None
  } else {
    map.get(&selected.trim().to_string()).cloned()
  })
}

pub fn dmenu_raw(lines: Vec<String>) -> std::io::Result<String> {
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
