use clap::Parser;
use rcli::{Opts, Command, process_csv};
// rcli csv -i input.csv -o output.json --header -d ','

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    println!("{:?}", opts);
    match opts.cmd {
        Command::Csv(opts) => process_csv(&opts.input,&opts.output)?,
    }
    Ok(())
}


