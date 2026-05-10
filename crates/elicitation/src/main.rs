use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let cli = elicitation::cli::Cli::parse();
    elicitation::cli::execute(cli)
}
