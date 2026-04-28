use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let cli = elicit_proofs::cli::Cli::parse();
    elicit_proofs::cli::execute(cli)
}
