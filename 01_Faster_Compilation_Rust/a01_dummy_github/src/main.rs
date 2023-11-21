use clap::Parser;

use ghk::Ghk;

mod ghk;
mod pr;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Ghk,
}

fn main() {
    let cli: Cli = Cli::parse();
    cli.command.exec();
}
