use clap::{ArgMatches, Command};
use rey::console::Console;

pub mod console;

fn rey() -> ArgMatches {
    Command::new("rey")
        .version("0.1.0")
        .author("Seido <seido@hackiado.com>")
        .about("about")
        .subcommand(Command::new("clean").about("Clean the project"))
        .subcommand(Command::new("init").about("Initialize the project"))
        .subcommand(Command::new("serve").about("Serve the project"))
        .subcommand(Command::new("watch").about("Watch the project"))
        .subcommand(Command::new("scan").about("Scan rey projects"))
        .get_matches()
}
fn main() {
    let matches = rey();
    match matches.subcommand() {
        Some(("clean", _sub_m)) => {
            Console::clean();
        }
        Some(("init", _)) => {
            Console::init();
        }
        Some(("serve", _)) => {
            Console::serve();
        }
        Some(("watch", _)) => {
            Console::watch().expect("Failed to watch files");
        }
        Some(("scan", _)) => {
            Console::scan();
        }
        _ => {
            println!("No valid subcommand was used. Use --help for more information.");
        }
    }
}
