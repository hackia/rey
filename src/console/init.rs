use crate::console::{Shell, console::Capsule, prompts::Question, views::admin::ok_clear};
use clap::Args;
use std::path::Path;

#[doc = "Initialize a new project with the given name."]
#[derive(Args)]
pub struct Init {
    #[arg(short, long)]
    pub name: Option<String>,
}

impl Shell for Init {
    fn run(&self) {
        let proj = Question::ask(
            "Project name",
            self.name.as_deref(),
            None, // or provide the third argument as needed
        )
        .expect("Failed to get project name");
        if Path::new(&proj).exists() {
            ok_clear(
                format!("Error: Directory '{proj}' already exists.").as_str(),
                false,
            );
            return;
        }
        match Capsule::init_project(&proj) {
            Ok(_) => ok_clear(
                format!("Project '{proj}' initialized successfully!").as_str(),
                false,
            ),
            Err(e) => ok_clear(format!("Error initializing project: {e}").as_str(), false),
        }
    }
}
