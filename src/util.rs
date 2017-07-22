use std::fs::File;
use std::io::{Result as ioResult, Read};

pub fn read_file_content(filename: &str) -> ioResult<String> {
  let mut file = File::open(filename)?;
  let mut file_content = String::new();

  file.read_to_string(&mut file_content)?;

  Ok(file_content)
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::env;

  #[test]
  fn test_read_file_content() {
    let cargoTomlPath =
      &(String::from(env::current_dir().unwrap().to_str().unwrap()) + "/Cargo.toml");

    assert!(read_file_content(cargoTomlPath).unwrap().contains("ladder"));
  }
}
