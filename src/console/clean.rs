use clap::Args;

use crate::console::{Shell, console::Capsule};

#[doc = "Clean the project directory by removing build artifacts and temporary files."]
#[derive(Args)]
pub struct Clean;

// Chaque commande implémente le trait `Capsule`.
impl Shell for Clean {
    fn run(&self) {
        Capsule::clean();
    }
}
