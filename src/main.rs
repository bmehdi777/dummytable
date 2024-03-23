use clap::Parser;
use dummytable::cli;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let args = cli::Cli::parse();

    args.handle();

    Ok(())
}
