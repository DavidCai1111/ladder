use std::error::Error;
use serde_json;
use util::read_file_content;

#[derive(Deserialize)]
pub struct Config {
  local_addr: Option<String>,
  server_addr: Option<String>,
  password: Option<String>,
  method: Option<String>,
  timeout: Option<u32>,
}

pub fn parse(filename: &str) -> Result<Config, Box<Error>> {
  let mut config: Config = serde_json::from_str(&read_file_content(filename)?)?;

  if config.local_addr.is_none() {
    config.local_addr = Some(String::from("0.0.0.0:6009"))
  }
  if config.server_addr.is_none() {
    config.server_addr = Some(String::from("0.0.0.0:8089"))
  }
  if config.password.is_none() {
    config.password = Some(String::from("ladder-password"))
  }
  if config.method.is_none() {
    config.method = Some(String::from("aes-128-cfb"))
  }
  if config.timeout.is_none() {
    config.timeout = Some(300)
  }

  Ok(config)
}
