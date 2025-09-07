use crate::console::{Shell, console::Capsule};
use clap::Args;

#[doc = "Start a local development server to serve the project."]
#[derive(Args)]
pub struct Serve;

impl Shell for Serve {
    fn run(&self) {
        Capsule::serve();
    }
}
