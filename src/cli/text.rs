use super::verify_file;
use clap::{Parser, Subcommand};
use std::{fmt::Display, str::FromStr};

#[derive(Subcommand, Debug)]
pub enum TextSubcommand {
    #[command(name = "sign", about = "Sign a message with private/share key")]
    Sign(TextSignOpts),

    #[command(name = "verify", about = "Verify a signed message")]
    Verify(TextVerifyOpts),
}

#[derive(Parser, Debug)]
pub struct TextSignOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,

    #[arg(long, short, value_parser = verify_file)]
    pub key: String,

    #[arg(long, value_parser = parse_sign_format, default_value = "blake3")]
    pub format: TextSignFormat,
}

#[derive(Parser, Debug)]
pub struct TextVerifyOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,

    #[arg(long, short, value_parser = verify_file)]
    pub key: String,

    #[arg(long, short)]
    pub signed: String,

    #[arg(long, value_parser = parse_sign_format, default_value = "blake3")]
    pub format: TextSignFormat,
}

#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    BLAKE3,
    ED25519,
}

fn parse_sign_format(format: &str) -> Result<TextSignFormat, anyhow::Error> {
    format.parse()
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(TextSignFormat::BLAKE3),
            "ed25519" => Ok(TextSignFormat::ED25519),
            _ => Err(anyhow::anyhow!("invalid format")),
        }
    }
}

impl From<TextSignFormat> for &'static str {
    fn from(item: TextSignFormat) -> Self {
        match item {
            TextSignFormat::BLAKE3 => "blake3",
            TextSignFormat::ED25519 => "ed25519",
        }
    }
}

impl Display for TextSignFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: &str = (*self).into();
        write!(f, "{}", s)
    }
}
