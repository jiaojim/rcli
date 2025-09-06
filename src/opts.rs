use clap::Parser;
use clap::Subcommand;
use std::fmt::Display;
use std::path::Path;
use std::str::FromStr;

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
}

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    JSON,
    YAML,
}

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,

    #[arg(short, long)]
    pub output: Option<String>,

    #[arg(long, value_parser = parse_format, default_value = "json")]
    pub format: OutputFormat,

    #[arg(long, default_value_t = true)]
    pub header: bool,

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
}

#[derive(Parser, Debug)]
pub struct GenPassOpts {
    #[arg(short, long, default_value_t = 16)]
    pub length: u8,

    #[arg(long, default_value_t = true)]
    pub uppercase: bool,

    #[arg(long, default_value_t = true)]
    pub lowercase: bool,

    #[arg(short, long, default_value_t = true)]
    pub number: bool,

    #[arg(short, long, default_value_t = true)]
    pub symbol: bool,
}

fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist!")
    }
}

fn parse_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    format.parse()
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(OutputFormat::JSON),
            "yaml" => Ok(OutputFormat::YAML),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}

impl From<OutputFormat> for &'static str {
    fn from(value: OutputFormat) -> Self {
        match value {
            OutputFormat::JSON => "json",
            OutputFormat::YAML => "yaml",
        }
    }
}

impl Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: &str = (*self).into();
        write!(f, "{}", s)
    }
}
