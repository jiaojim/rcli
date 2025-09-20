use clap::{Parser, command};
use rcli::{
    process_csv, process_decode, process_encode, process_genpass, process_sign, process_verify, Base64Subcommand, Command, Opts, TextSignFormat, TextSubcommand
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

        Command::Text(subcmd) => match subcmd {
            TextSubcommand::Sign(opts) => {
                 match opts.format {
                     TextSignFormat::BLAKE3 => {
                        process_sign(&opts.input, &opts.key, opts.format)?;
                     },
                     TextSignFormat::ED25519 =>{}
                 }
            },
            TextSubcommand::Verify(opts) =>{
                 match opts.format {
                     TextSignFormat::BLAKE3 => {
                        process_verify(&opts.input, &opts.key, opts.format,&opts.signed)?;
                     },
                     TextSignFormat::ED25519 =>{}
                 }
            }
        }
    }

    Ok(())
}
