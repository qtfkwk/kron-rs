use clap::Parser;
use kron_lib::*;

#[derive(Parser)]
#[command(about, version, max_term_width = 80)]
struct Cli {
    args: Vec<String>,
}

fn main() {
    let cli = Cli::parse();
    if cli.args.is_empty() {
        println!("{}", Kron::now());
    }
}
