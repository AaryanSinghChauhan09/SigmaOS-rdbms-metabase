#![forbid(unsafe_code)]
use clap::{Parser, Subcommand};
#[derive(Parser)]
#[command(name = "rdbms-metabase", about = "SigmaOS RDBMS Metabase")]
struct Cli { #[command(subcommand)] cmd: Commands }
#[derive(Subcommand)]
enum Commands { List, Status }
fn main() {
    let cli = Cli::parse();
    match cli.cmd {
        Commands::List   => println!("rdbms-metabase: no entries configured."),
        Commands::Status => println!("rdbms-metabase: ready."),
    }
}
