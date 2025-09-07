use inquire::Editor;

use crate::console::{Shell, console::Capsule, views::admin::ok_clear};

#[doc = "Edit configuration files such as Rocket.toml, .env, Cargo.toml, etc."]
#[derive(clap::Args)]
pub struct Edit;

impl Shell for Edit {
    fn run(&self) {
        let files = vec![
            "Rocket.toml",
            ".env",
            ".env.example",
            "Cargo.toml",
            "package.json",
            "tsconfig.json",
        ];
        let files = inquire::MultiSelect::new("Select a file to edit:", files)
            .prompt()
            .expect("Failed to get user input");
        if files.is_empty() {
            ok_clear("No file selected, exiting.", false);
            return;
        }
        for file in files {
            if let Err(e) = Capsule::edit(file) {
                ok_clear(format!("Error editing {file}: {e}").as_str(), false);
            }
            let initial_content = std::fs::read_to_string(file).unwrap_or_default();
            let editor = Editor::new("Edit").with_predefined_text(&initial_content);
            let text = editor
                .prompt()
                .map_err(|e| {
                    ok_clear(format!("Error opening editor: {e}").as_str(), false);
                })
                .expect("Failed to open editor");
            std::fs::write(file, text).expect("Failed to write file");
            ok_clear(format!("{file} updated successfully!").as_str(), false);
        }
    }
}
