use clap::Parser;
use rcli::{
    Base64Subcommand, Command, Opts, process_csv, process_decode, process_encode, process_genpass,
};

fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();
    match opts.cmd {
        Command::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?
        }

        Command::GenPass(opts) => process_genpass(
            opts.length,
            opts.uppercase,
            opts.lowercase,
            opts.number,
            opts.symbol,
        )?,

        Command::Base64Subcommand(subcmd) => match subcmd {
            Base64Subcommand::Decode(opts) => process_decode(&opts.input, opts.format)?,
            Base64Subcommand::Encode(opts) => process_encode(&opts.input, opts.format)?,
        },
    }

    Ok(())
}
