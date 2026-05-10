#[cfg(feature = "runner")]
fn main() -> anyhow::Result<()> {
    use clap::Parser;
    let cli = elicit_proofs::cli::Cli::parse();
    elicit_proofs::cli::execute(cli)
}

#[cfg(not(feature = "runner"))]
fn main() {}
