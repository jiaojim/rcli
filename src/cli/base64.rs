use std::{fmt::Display, str::FromStr};

use super::verify_input_file;
use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug)]
pub enum Base64Subcommand {
    #[command(name = "decode", about = "Decode a base64 string")]
    Decode(DecodeOpts),

    #[command(name = "encode", about = "Encode a string to base64")]
    Encode(EncodeOpts),
}

#[derive(Parser, Debug)]
pub struct DecodeOpts {
    #[arg(short, long, value_parser = verify_input_file, default_value = "-")]
    pub input: String,

    #[arg(long, value_parser = parse_base64_format, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Parser, Debug)]
pub struct EncodeOpts {
    #[arg(short, long, value_parser = verify_input_file, default_value = "-")]
    pub input: String,

    #[arg(long, value_parser = parse_base64_format, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

fn parse_base64_format(format: &str) -> Result<Base64Format, anyhow::Error> {
    format.parse()
}

impl FromStr for Base64Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::UrlSafe),
            _ => Err(anyhow::anyhow!("invalid format")),
        }
    }
}

impl From<Base64Format> for &'static str {
    fn from(item: Base64Format) -> Self {
        match item {
            Base64Format::Standard => "standard",
            Base64Format::UrlSafe => "urlsafe",
        }
    }
}

impl Display for Base64Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: &str = (*self).into();
        write!(f, "{}", s)
    }
}
