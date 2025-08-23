use clap::Parser;
use rcli::{Opts, Command, process_csv};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        Command::Csv(opts) => process_csv(&opts.input, &opts.output)?
    }
    Ok(())
}


