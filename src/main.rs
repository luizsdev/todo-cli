use std::io;

use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Task {
    description: String,
    completed: bool,
}

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand)]
enum Commands {
    Add { description: String },
    List,
    Complete { index: usize },
    Delete { index: usize },
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();
}
