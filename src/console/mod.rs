use clap::Subcommand;

pub trait Shell {
    fn run(&self);
}

pub mod clean;
pub mod console;
pub mod edit;
pub mod init;
pub mod prompts;
pub mod runs;
pub mod scan;
pub mod serve;
pub mod views;
pub mod watch;
pub mod watchers;

#[derive(Subcommand)]
pub enum Commands {
    Clean(clean::Clean),
    Edit(edit::Edit),
    Init(init::Init),
    Run(runs::Run),
    Scan(scan::Scan),
    Watch(watch::Watch),
    Serve(serve::Serve),
}
