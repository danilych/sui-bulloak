mod scaffold;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "sui-bulloak", version, about = "A scaffolding tool for generating Move specification files to speed up formal verification on Sui with sui-prover.")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Scaffold {
        #[arg(short, long)]
        path: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scaffold { path } => {
            scaffold::scaffold(path);
        }
    }
}