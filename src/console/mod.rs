use std::{
    fs::{File, remove_dir_all, remove_file},
    path::Path,
};

use crate::console::{
    prompts::Question,
    views::{
        admin::{ok_clear, ok_command},
        web::{
            generate_admin,
            generate_admin_view, generate_web, generate_web_view, init_all, is_initialized, scan,
        },
    }, watchers::watch,
};

pub mod prompts;
pub mod views;
pub mod watchers;
pub struct Console;

impl Console {
    pub fn scan() {
        ok_clear("Scanning for rey projects...", false);
        scan();
        ok_clear("Scan complete!", false);
    }
    pub fn init() {
        if is_initialized() {
            ok_clear("Project is already initialized.", false);
            return;
        } else {
            ok_clear("Initialization of project", false);
        }
        if Question::confirm(
            "Do you want to create a new project in the current directory?",
            Some("y"),
            Some("Confirm initialization"),
        )
        .expect("failed to get confirmation")
        {
            ok_clear("initializing project...", false);
            init_all().expect("failed to initialize project");
            ok_clear("project initialized successfully!", false);
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
            if Path::new("logs/").exists() {
                remove_dir_all("logs/").expect("failed to remove logs directory");
            }
            if Path::new("front/").exists() {
                remove_dir_all("front/").expect("failed to remove front directory");
            }
            if Path::new("templates/").exists() {
                remove_dir_all("templates/").expect("failed to remove templates directory");
            }
            if Path::new("src/").exists() {
                remove_dir_all("src/").expect("failed to remove src directory");
            }
            if Path::new("Cargo.toml").exists() {
                remove_file("Cargo.toml").expect("failed to remove Cargo.toml");
            }
            if Path::new("Rocket.toml").exists() {
                remove_file("Rocket.toml").expect("failed to remove Rocket.toml");
            }
            if Path::new(".hgignore").exists() {
                remove_file(".hgignore").expect("failed to remove .hgignore");
            }
            if Path::new("README.md").exists() {
                remove_file("README.md").expect("failed to remove README.md");
            }
            if Path::new("LICENSE").exists() {
                remove_file("LICENSE").expect("failed to remove LICENSE");
            }
            if Path::new("tsconfig.json").exists() {
                remove_file("tsconfig.json").expect("failed to remove tsconfig.json");
            }
            if Path::new("package.json").exists() {
                remove_file("package.json").expect("failed to remove package.json");
            }
            if Path::new("tests/").exists() {
                remove_dir_all("tests/").expect("failed to remove tests directory");
            }
            if Path::new("public/").exists() {
                remove_dir_all("public/").expect("failed to remove public directory");
            }
            if Path::new("target/").exists() {
                remove_dir_all("target/").expect("failed to remove target directory");
            }
            if Path::new("Cargo.lock").exists() {
                remove_file("Cargo.lock").expect("failed to remove Cargo.lock");
            }
            if Path::new("node_modules/").exists() {
                remove_dir_all("node_modules/").expect("failed to remove node_modules directory");
            }
            if Path::new("yarn.lock").exists() {
                remove_file("yarn.lock").expect("failed to remove yarn.lock");
            }
            if Path::new("pnpm-lock.yaml").exists() {
                remove_file("pnpm-lock.yaml").expect("failed to remove pnpm-lock.yaml");
            }
            ok_clear("existing project files removed!", false);
        } else {
            ok_clear("removal of project files cancelled", false);
        }
    }

    fn update_running() {
        if Path::new("front/web/.running").exists() {
            ok_clear("Cleaning previous build...", false);
            remove_file("front/web/.running").expect("failed to remove .running file");
            remove_file("front/admin/.running").expect("failed to remove .running file");
            ok_clear("Previous build cleaned!", false);
        } else {
            ok_clear("No previous build found.", false);
            File::create("front/web/.running").expect("failed to create .running file");
            File::create("front/admin/.running").expect("failed to create .running file");
            ok_clear(".running file created!", false);
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
        if !is_initialized() {
            ok_clear(
                "Project is not initialized. Please run 'rey init' first.",
                false,
            );
            return;
        }
        Console::update_running();
        ok_command(
            "serving the site...",
            true,
            std::process::Command::new("cargo").arg("run"),
        );
    }

    pub fn watch() -> anyhow::Result<()> {
        watch().expect("msg");
        Ok(())
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
                "compiling web scss",
                false,
                std::process::Command::new("npx")
                    .arg("sass")
                    .arg("front/web/scss/web.scss:public/css/web.css")
                    .arg("--style=compressed"),
            );
            ok_command(
                "compiling admin scss",
                false,
                std::process::Command::new("npx")
                    .arg("sass")
                    .arg("front/admin/scss/admin.scss:public/css/admin.css")
                    .arg("--style=compressed"),
            );
            ok_clear("scss compiled successfully!", false);
        }

        if extensions.contains(&"rs") {
            ok_command(
                "checking rust code",
                false,
                std::process::Command::new("cargo").arg("clippy"),
            );
            ok_clear("rust code checked successfully!", false);
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
