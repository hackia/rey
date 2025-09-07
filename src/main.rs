use clap::{Parser, command};
use rey::console::{Commands, Shell};
pub mod console;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
impl Shell for Cli {
    fn run(&self) {
        match &self.command {
            Commands::Clean(cmd) => cmd.run(),
            Commands::Edit(cmd) => cmd.run(),
            Commands::Init(cmd) => cmd.run(),
            Commands::Run(cmd) => cmd.run(),
            Commands::Scan(cmd) => cmd.run(),
            Commands::Watch(cmd) => cmd.run(),
            Commands::Serve(cmd) => cmd.run(),
        }
    }
}

fn main() {
    let cli: Cli = Cli::parse();

    cli.run();
}
