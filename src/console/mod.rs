use std::fs::{remove_dir_all, remove_file};

use crate::console::{
    prompts::Question,
    views::{
        admin::{ok_clear, ok_command},
        web::{generate_admin, generate_admin_view, generate_web, generate_web_view, init_all},
    },
};

pub mod prompts;
pub mod views;

pub struct Console;

impl Console {
    pub fn init() {
        if Question::confirm(
            "Do you want to create a new project in the current directory?",
            Some("y"),
            Some("Confirm initialization"),
        )
        .expect("failed to get confirmation")
        {
            ok_clear("initializing project...", false);
            init_all().expect("failed to initialize project");
            ok_clear("project initialized successfully!", true);
        } else {
            ok_clear("project initialization cancelled", false);
        }
    }

    pub fn clean() {
        if Question::confirm(
            "Are you sure you want to remove existing project files? This action cannot be undone.",
            Some("n"),
            Some("Confirm removal"),
        )
        .expect("failed to get confirmation")
        {
            ok_clear("removing existing project files...", false);
            if std::path::Path::new("front/").exists() {
                remove_dir_all("front/").expect("failed to remove front directory");
            }
            if std::path::Path::new("templates/").exists() {
                remove_dir_all("templates/").expect("failed to remove templates directory");
            }
            if std::path::Path::new("src/").exists() {
                remove_dir_all("src/").expect("failed to remove src directory");
            }
            if std::path::Path::new("Cargo.toml").exists() {
                remove_file("Cargo.toml").expect("failed to remove Cargo.toml");
            }
            if std::path::Path::new("Rocket.toml").exists() {
                remove_file("Rocket.toml").expect("failed to remove Rocket.toml");
            }
            if std::path::Path::new(".hgignore").exists() {
                remove_file(".hgignore").expect("failed to remove .hgignore");
            }
            if std::path::Path::new("README.md").exists() {
                remove_file("README.md").expect("failed to remove README.md");
            }
            if std::path::Path::new("LICENSE").exists() {
                remove_file("LICENSE").expect("failed to remove LICENSE");
            }
            if std::path::Path::new("tsconfig.json").exists() {
                remove_file("tsconfig.json").expect("failed to remove tsconfig.json");
            }
            if std::path::Path::new("package.json").exists() {
                remove_file("package.json").expect("failed to remove package.json");
            }
            if std::path::Path::new("tests/").exists() {
                remove_dir_all("tests/").expect("failed to remove tests directory");
            }
            ok_clear("existing project files removed!", false);
        } else {
            ok_clear("removal of project files cancelled", false);
        }
    }

    /// Create a web view
    pub fn create_web_view() {
        println!("Creating a web view...");
        let view = Question::ask("What is the view name?", None, Some("Enter the view name"))
            .expect("msg");
        if Question::confirm(
            "Do you want to create all files for this view?",
            Some("y"),
            Some("Confirm creation"),
        )
        .expect("failed to get confirmation")
        {
            ok_clear("creating all view files...", false);
            generate_web(&view).expect("failed to create view files");
            ok_clear("all view files created successfully!", true);
        } else {
            ok_clear("creating all view files cancelled", false);
            generate_web_view(&view).expect("failed to create web view file");
            ok_clear("web view file created successfully!", true);
        }
        println!("View created: {}", view);
    }

    pub fn serve() {
        ok_clear("Serving the site locally...", true);
        ok_command(
            "compiling",
            true,
            std::process::Command::new("cargo").arg("run"),
        );
    }

    pub fn watch() {
        ok_clear("Watching for changes...", true);
        Console::compile(["ts", "scss", "rs"]);
        Console::serve();
    }

    pub fn compile(extensions: [&str; 3]) {
        if extensions.contains(&"ts") {
            ok_command(
                "compiling typescript",
                false,
                std::process::Command::new("npx")
                    .arg("tsc")
                    .arg("--project")
                    .arg("tsconfig.json"),
            );
            ok_clear("typescript compiled successfully!", false);
        }
        if extensions.contains(&"scss") {
            ok_command(
                "compiling scss",
                false,
                std::process::Command::new("npx")
                    .arg("sass")
                    .arg("src/assets/scss:public/assets/css")
                    .arg("--style=compressed"),
            );
            ok_clear("scss compiled successfully!", false);
        }
        if extensions.contains(&"rs") {
            ok_command(
                "compiling rust",
                false,
                std::process::Command::new("cargo-watch")
                    .arg("-x")
                    .arg("run"),
            );
            ok_clear("rust compiled successfully!", false);
        }
    }

    /// Create an admin view
    pub fn create_admin_view() {
        println!("Creating an admin view...");
        let view = Question::ask("What is the view name?", None, Some("Enter the view name"))
            .expect("msg");
        if Question::confirm(
            "Do you want to create all files for this view?",
            Some("y"),
            Some("Confirm creation"),
        )
        .expect("failed to get confirmation")
        {
            ok_clear("creating all view files...", false);
            generate_admin(&view).expect("failed to create view files");
            ok_clear("all view files created successfully!", true);
        } else {
            ok_clear("creating all view files cancelled", false);
            generate_admin_view(&view).expect("failed to create admin view file");
            ok_clear("admin view file created successfully!", true);
        }
        println!("View created: {}", view);
    }
}
