use std::error::Error;
use serde_json;
use util::read_file_content;

#[derive(Deserialize, Debug)]
pub struct Config {
    local_addr: String,
    server_addr: String,
    password: String,
    method: String,
    timeout: u32,
}

pub fn parse(filename: &str) -> Result<Config, Box<Error>> {
    let config: Config = serde_json::from_str(&read_file_content(filename)?)?;

    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_parse() {
        let configPath =
            &(String::from(env::current_dir().unwrap().to_str().unwrap()) + "/test/config.json");
        let config: Config = parse(configPath).unwrap();

        assert_eq!(config.local_addr, String::from("0.0.0.0:6010"));
        assert_eq!(config.server_addr, String::from("0.0.0.0:8089"));
        assert_eq!(config.password, String::from("ladder-password"));
        assert_eq!(config.method, String::from("aes-128-cfb"));
        assert_eq!(config.timeout, 300);
    }
}
