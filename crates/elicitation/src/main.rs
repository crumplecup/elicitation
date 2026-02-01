use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let cli = elicitation::cli::Cli::parse();
    elicitation::cli::execute(cli)
}
