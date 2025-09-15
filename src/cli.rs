mod base64;
mod csv;
mod genpass;

use std::path::Path;

pub use self::base64::{Base64Format, Base64Subcommand};
pub use self::csv::{CsvOpts, OutputFormat};
use clap::{Parser, Subcommand};
pub use genpass::GenPassOpts;

#[derive(Parser, Debug)]
#[command(name = "rcli", version)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),

    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),

    #[command(name = "base64", subcommand, about = "Base64 Encode or Decode")]
    Base64Subcommand(Base64Subcommand),
}

fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist!")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_input_file("-"), Ok("-".into()));
        assert_eq!(verify_input_file("*"), Err("File does not exist!"));
        assert_eq!(verify_input_file("cargo.toml"), Ok("cargo.toml".into()));
        assert_eq!(verify_input_file("not-exist"), Err("File does not exist!"));
    }
}
