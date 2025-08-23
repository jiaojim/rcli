use clap::Parser;
use rcli::{Command, Opts, process_csv};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        Command::Csv(opts) => process_csv(&opts.input, &opts.output)?,
    }

    Ok(())
}
