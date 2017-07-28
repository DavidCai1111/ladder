use std::error::Error;
use serde_json;
use util::read_file_content;

#[derive(Deserialize, Debug)]
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
        config.local_addr = Some(String::from("0.0.0.0:6009"));
    }
    if config.server_addr.is_none() {
        config.server_addr = Some(String::from("0.0.0.0:8089"));
    }
    if config.password.is_none() {
        config.password = Some(String::from("ladder-password"));
    }
    if config.method.is_none() {
        config.method = Some(String::from("aes-128-cfb"));
    }
    if config.timeout.is_none() {
        config.timeout = Some(300);
    }

    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_parse() {
        let configPath = &(String::from(env::current_dir().unwrap().to_str().unwrap()) +
                           "/test/config.json");
        let config: Config = parse(configPath).unwrap();

        assert_eq!(config.local_addr, Some(String::from("0.0.0.0:6010")));
        assert_eq!(config.server_addr, Some(String::from("0.0.0.0:8089")));
        assert_eq!(config.password, Some(String::from("ladder-password")));
        assert_eq!(config.method, Some(String::from("aes-128-cfb")));
        assert_eq!(config.timeout, Some(300));
    }
}
