use crate::console::{Shell, console::Capsule};
use clap::Args;

#[doc = "Watch for file changes and automatically recompile the project."]
#[derive(Args)]
pub struct Watch;

impl Shell for Watch {
    fn run(&self) {
        Capsule::watch().expect("Failed to start watching files");
    }
}
