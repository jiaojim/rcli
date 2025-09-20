use std::{fs::File, io::Read};

use base64::engine::general_purpose::URL_SAFE;
use base64::prelude::*;

use crate::{get_reader, Base64Format};

pub fn process_encode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let mut reader: Box<dyn Read> = get_reader(input)?;
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let buf = buf.trim_end();

    let encoded = match format {
        Base64Format::Standard => BASE64_STANDARD.encode(&buf),
        Base64Format::UrlSafe => URL_SAFE.encode(&buf),
    };

    println!("{}", encoded);
    Ok(())
}

pub fn process_decode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let mut reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };
    let mut buf: String = String::new();
    reader.read_to_string(&mut buf)?;

    let buf = buf.trim_end();

    let decoded = match format {
        Base64Format::Standard => BASE64_STANDARD.decode(&buf)?,
        Base64Format::UrlSafe => URL_SAFE.decode(&buf)?,
    };

    let decoded = String::from_utf8(decoded)?;

    println!("{}", decoded);
    Ok(())
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_process_encode() {
       let input = "hello world!";
       let format = Base64Format::Standard;
       assert!(process_decode(input, format).is_ok());
    }

    #[test]
    fn test_process_decode() {
        
    }
}