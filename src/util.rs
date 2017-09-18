use std::fs::File;
use std::io::{Result as ioResult, Read};
use crypto::md5;
use crypto::digest::Digest;
use bytes::Bytes;

const MD5_LEN: usize = 32;

pub fn read_file_content(filename: &str) -> ioResult<String> {
    let mut file = File::open(filename)?;
    let mut file_content = String::new();

    file.read_to_string(&mut file_content)?;

    Ok(file_content)
}

pub fn md5_sum(bytes: &Bytes) -> Bytes {
    let mut hash = md5::Md5::new();
    hash.input(bytes);
    Bytes::from(hash.result_str())
}

pub fn generate_key(password: &str, key_len: usize) -> Bytes {
    let count = (key_len as f64 / MD5_LEN as f64).ceil() as usize;
    let md5_result = md5_sum(&Bytes::from(password));
    let mut result = md5_result.clone();

    for _ in 1..count {
        result.extend_from_slice(&md5_result);
    }

    Bytes::from(&result[..key_len])
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
        let result = md5_sum(&Bytes::from("test"));

        assert_eq!(result, String::from("098f6bcd4621d373cade4e832627b4f6"));
    }

    #[test]
    fn test_generate_key_16() {
        let mut sum = String::from("5d41402abc4b2a76b9719d911017c592");
        assert_eq!(generate_key("hello", 32), sum);
    }

    #[test]
    fn test_generate_key_32() {
        let mut sum = String::from("5d41402abc4b2a76b9719d911017c5925d41402abc4b2a76b9719d911017c592");
        assert_eq!(generate_key("hello", 64), sum);
    }
}
