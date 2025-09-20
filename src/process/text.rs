use anyhow::{Ok, Result};
use base64::{Engine, prelude::BASE64_URL_SAFE_NO_PAD};
use std::{fs, io::Read};

use crate::{cli::TextSignFormat, get_reader};

pub fn process_sign(input: &str, key: &str, format: TextSignFormat) -> Result<()> {
    let mut reader: Box<dyn Read> = get_reader(input)?;
    let hash = match format {
        TextSignFormat::BLAKE3 => {
            let key: Vec<u8> = fs::read(key)?;
            let key = &key[..32];
            let key: [u8; 32] = key.try_into()?;
            let signer = Blake3 { key };
            signer.Sign(&mut reader)?
        }
        TextSignFormat::ED25519 => {
            todo!()
        }
    };
    let signed = BASE64_URL_SAFE_NO_PAD.encode(&hash);
    println!("{}", signed);
    Ok(())
}

pub fn process_verify(input: &str, key: &str, format: TextSignFormat, sig: &str) -> Result<()> {
    let mut reader: Box<dyn Read> = get_reader(input)?;
    let ok = match format {
        TextSignFormat::BLAKE3 => {
            let key: Vec<u8> = fs::read(key)?;
            let key = &key[..32];
            let key: [u8; 32] = key.try_into()?;
            let signer = Blake3 { key };
            signer.Verify(&mut reader, sig.as_bytes())?
        }
        TextSignFormat::ED25519 => {
            todo!()
        }
    };
    println!("{}", ok);
    Ok(())
}

trait TextSigner {
    fn Sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;

    fn Verify<T: Read>(&self, reader: T, signed: &[u8]) -> Result<bool>;
}

struct Blake3 {
    key: [u8; 32],
}

impl TextSigner for Blake3 {
    fn Sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        Ok(blake3::keyed_hash(&self.key, &buf).as_bytes().to_vec())
    }

    fn Verify<T: Read>(&self, mut reader: T, signed: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let hash = blake3::keyed_hash(&self.key, &buf);
        let hash = hash.as_bytes();
        Ok(hash == signed)
    }
}

struct Ed25519Dalek {
    key: [u8; 32],
}

impl TextSigner for Ed25519Dalek {
    fn Sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        todo!()
    }

    fn Verify<T: Read>(&self, reader: T, signed: &[u8]) -> Result<bool> {
        todo!()
    }
}
