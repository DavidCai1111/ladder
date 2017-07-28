use std::fs::File;
use std::io::{Result as ioResult, Read};
use crypto::md5;
use crypto::digest::Digest;

pub fn read_file_content(filename: &str) -> ioResult<String> {
    let mut file = File::open(filename)?;
    let mut file_content = String::new();

    file.read_to_string(&mut file_content)?;

    Ok(file_content)
}

pub fn md5_sum(bytes: &[u8]) -> String {
    let mut hash = md5::Md5::new();
    hash.input(bytes);
    hash.result_str()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_read_file_content() {
        let cargoTomlPath = &(String::from(env::current_dir().unwrap().to_str().unwrap()) +
                              "/Cargo.toml");

        assert!(read_file_content(cargoTomlPath).unwrap().contains("ladder"));
    }


    #[test]
    fn test_md5_sum() {
        let result = md5_sum(String::from("test").as_bytes());

        assert_eq!(result, String::from("098f6bcd4621d373cade4e832627b4f6"));
    }
}
