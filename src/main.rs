use crate::commands::Commands;
use clap::Parser;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Encode(args) => { commands::encode(args) }
        Commands::Decode(args) => { commands::decode(args) }
        Commands::Remove(args) => { commands::remove(args) },
        Commands::Print(args) => { commands::print(args) }
    }
}
