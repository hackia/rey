use std::{
    fs::{File, create_dir_all},
    io::Write,
};

use crate::console::{
    Shell,
    prompts::Question,
    views::admin::{ok_clear, ok_command},
};
use clap::Args;
use std::process::Command;

#[doc = "Generate static files from the web console."]
#[derive(Args)]
pub struct Generate;

pub fn generate_controller(name: &str) {
    ok_clear("Generating controllers...", true);
    create_dir_all("src/controllers").expect("failed to create controllers directory");
    let mut c = File::create(format!("src/controllers/{}_controller.rs", name))
        .expect("failed to create controller file");
    c.write_all(
        b"use rocket::response::Response;
        use rocket::http::Status;
        use rocket::get;
        use rocket::post;
        use rocket::request::{FromRequest, Outcome, Request};
        
        fn before_request() -> Result<(), Status> {
            // Middleware logic here (e.g., authentication)
            Ok(())
        }
        
        #[derive(Debug)]
        pub struct {Name}Controller;
        
        impl {Name}Controller {
            pub fn new() -> Self {
                Self {
                    // Initialize fields here
                }
            }
            
            /// List all {name}s
            /// GET /{name}
            #[get(\"/{name}\")]
            pub fn index(&self) {
                // List all {name}s
            }
            
            /// Show a specific {name}
            /// GET /{name}/<id>
            #[get(\"/show/{name}/<id>\")]
            pub fn show(&self, id: usize) {
                // Show a specific {name}
            }

            /// Create a new {name}
            /// POST /{name}
            #[post(\"/create/{name}\")]
            pub fn create(&self) {
                // Create a new {name}
            }
            
            /// Update an existing {name}
            #[post(\"/update/{name}/<id>\")]
            pub fn update(&self, id: usize) {
                // Update an existing {name}
            }

            /// Delete a {name}
            #[post(\"/delete/{name}/<id>\")]
            pub fn delete(&self, id: usize) -> Response {
                // Delete a {name}
            }
        }
    ",
    )
    .expect("failed to write to controller file");
    c.sync_all().expect("failed to sync controller file");
    ok_clear("Controller generated successfully", false);
}

pub fn generate_model(name: &str) {
    ok_clear("Generating model...", false);
    create_dir_all("src/models").expect("failed to create models directory");
    File::create(format!("src/models/{name}_model.rs")).expect("failed to create model file");
    ok_clear("Model generated successfully", false);
}

pub fn generate_view(name: &str) {
    ok_clear("Generating view...", false);
    create_dir_all(format!("templates/{name}/")).expect("failed to create views directory");
    File::create(format!("templates/{name}/{name}.html.tera")).expect("failed to create view file");
    ok_clear("View generated successfully", false);
}
pub fn generate_migration(name: &str) {
    let mut cmd = Command::new("diesel");
    cmd.args(["migration", "generate", name]);
    ok_command("Generating migration...", false, &mut cmd);
    ok_clear("Migration generated successfully", false);
    ok_clear(
        "Edit the generated migration files in the migrations directory. Press Enter to continue.",
        false,
    );
    let _ = std::io::stdin().read_line(&mut String::new());
}

impl Shell for Generate {
    fn run(&self) {
        let mut valid = vec![
            "controllers".to_string(),
            "models".to_string(),
            "views".to_string(),
            "migrations".to_string(),
            "all".to_string(),
            "none".to_string(),
        ];
        let name = Question::ask("Name of the component", None, None).expect("failed to get input");
        valid.sort();
        let todo = Question::select(
            "What do you want to generate?",
            &valid.to_vec(),
            Some("Select a type to generate"),
        )
        .expect("failed to get input");
        match todo.as_str() {
            "controllers" => generate_controller(&name),
            "models" => generate_model(&name),
            "views" => generate_view(&name),
            "migrations" => generate_migration(&name),
            "all" => {
                ok_clear("Generating all components...", false);
                generate_controller(&name);
                generate_model(&name);
                generate_view(&name);
                generate_migration(&name);
                ok_clear("All components generated successfully.", false);
            }
            "none" => {
                ok_clear("No components generated.", false);
            }
            _ => {
                ok_clear("Invalid option selected.", false);
            }
        }
    }
}
