use clap::Args;

use crate::console::{Shell, console::Capsule};

#[doc = "Scan home directory in order to find rey projects."]
#[derive(Args)]
pub struct Scan;

impl Shell for Scan {
    fn run(&self) {
        Capsule::scan();
    }
}
