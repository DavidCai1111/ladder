use std::fs::File;
use std::io::{Result as ioResult, Read};

pub fn read_file_content(filename: &str) -> ioResult<String> {
  let mut file = File::open(filename)?;
  let mut file_content = String::new();

  file.read_to_string(&mut file_content)?;

  Ok(file_content)
}
