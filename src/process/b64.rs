use anyhow::Result;
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE},
    Engine as _,
};
use std::fs::File;
use std::io::Read;

use crate::Base64Format;
// cargo run -- base64 encode --format urlsafe
pub fn process_encode(input: &str, format: Base64Format) -> Result<()> {
    let mut reader = get_reader(input)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let encoded = match format {
        Base64Format::Standard => STANDARD.encode(&buf),
        Base64Format::UrlSafe => URL_SAFE.encode(&buf),
    };

    println!("{}", encoded);
    Ok(())
}

pub fn process_decode(input: &str, format: Base64Format) -> Result<()> {
    let mut reader = get_reader(input)?;
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let buf = buf.trim();

    let decoded = match format {
        Base64Format::Standard => STANDARD.decode(&buf)?,
        Base64Format::UrlSafe => URL_SAFE.decode(&buf)?,
    };
    let decoded = String::from_utf8(decoded)?;
    println!("{:?}", decoded);
    Ok(())
}

fn get_reader(input: &str) -> Result<Box<dyn Read>> {
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin()) // 两个不同的数据类型，用Box来消除差异
    } else {
        Box::new(File::open(input)?)
    };
    Ok(reader)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process_encode() {
        let input = "Cargo.toml";
        let format = Base64Format::UrlSafe;
        assert!(process_encode(input, format).is_ok());
    }
    #[test]
    fn test_process_decode() {
        let input = "fixtures/b64.txt";
        let format = Base64Format::Standard;
        assert!(process_decode(input, format).is_ok());
    }
}
