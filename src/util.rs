use std::fs::File;
use std::io::{Result as ioResult, Read};
use crypto::md5;
use crypto::digest::Digest;
use bytes::Bytes;

const MD5_LEN: usize = 16;

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
    // TODO: optimize generate_key()
    let count = (key_len as f64 / MD5_LEN as f64).ceil() as usize;

    md5_sum(&Bytes::from(password));
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
}
