use anyhow::Result;
use clap::Parser;
use kron_lib::*;
use pager::Pager;

#[derive(Parser)]
#[command(about, version, max_term_width = 80)]
struct Cli {
    /// Format (COMPACT, ISO8601, ISO8601NS)
    #[arg(short, long, default_value = "ISO8601")]
    format: String,

    /// Print readme
    #[arg(short, long)]
    readme: bool,

    /// Argument ["%s[%.f]" timestamp, default: now]
    #[arg(name = "ARG")]
    args: Vec<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.readme {
        Pager::with_pager("bat -pl md").setup();
        print!("{}", include_str!("../README.md"));
        return Ok(());
    }

    let fmt = KronFormat::from(&cli.format)?;

    let mut timestamps = vec![];
    for arg in &cli.args {
        timestamps.push(Kron::from(arg)?);
    }
    if timestamps.is_empty() {
        timestamps.push(Kron::now());
    }

    for timestamp in timestamps {
        println!("{}", timestamp.format(&fmt)?);
    }

    Ok(())
}
