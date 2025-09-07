use clap::{Arg, ArgMatches, Command, builder::PossibleValuesParser};
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
        .subcommand(
            Command::new("edit").about("Edit Rocket.toml config").arg(
                Arg::new("file")
                    .help("File to edit")
                    .long("if")
                    .short('f')
                    .value_parser(PossibleValuesParser::new([
                        "Rocket.toml",
                        "Cargo.toml",
                        "package.json",
                        "tsconfig.json",
                    ]))
                    .default_missing_value("Rocket.toml")
                    .default_value("Rocket.toml")
                    .required(false),
            ),
        )
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
        Some(("edit", a)) => {
            Console::edit(a.get_one::<String>("file").unwrap().as_str())
                .expect("failed to edit file");
        }
        _ => {
            println!("No valid subcommand was used. Use --help for more information.");
        }
    }
}
