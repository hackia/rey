use crate::console::Shell;
use crate::console::views::admin::ok_clear;
use clap::Args;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::process::{Command as ShellCommand, Stdio};

#[derive(Deserialize)]
struct Config {
    scripts: Option<HashMap<String, String>>,
}

#[doc = "Run predefined scripts from rey.toml."]
#[derive(Args)]
pub struct Run {
    script_name: String,
}

impl Shell for Run {
    fn run(&self) {
        let content = match fs::read_to_string("rey.toml") {
            Ok(c) => c,
            Err(_) => {
                eprintln!("Error: rey.toml file not found in the current directory.");
                return;
            }
        };

        let config: Config = match toml::from_str(&content) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Error: Failed to parse rey.toml file. {e}");
                return;
            }
        };

        // 3. Find and execute the script
        if let Some(scripts) = config.scripts {
            if let Some(script_command) = scripts.get(&self.script_name) {
                ok_clear(
                    format!("Executing script: '{}'", self.script_name).as_str(),
                    false,
                );
                ok_clear(script_command.as_str(), false);

                let status = ShellCommand::new("sh")
                    .arg("-c")
                    .args(script_command.split_whitespace())
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .status();

                match status {
                    Ok(exit_status) if !exit_status.success() => {
                        ok_clear(
                            format!("\nError: Script '{}' failed.", self.script_name).as_str(),
                            false,
                        );
                    }
                    Err(e) => {
                        ok_clear(
                            format!("\nError: Failed to execute the script. {}", e).as_str(),
                            false,
                        );
                    }
                    _ => {
                        ok_clear(
                            format!("Script '{}' executed successfully.", self.script_name)
                                .as_str(),
                            false,
                        );
                    }
                }
            } else {
                eprintln!(
                    "Error: Script '{}' not found in rey.toml.",
                    self.script_name
                );
            }
        } else {
            eprintln!("Error: No [scripts] section found in rey.toml.");
        }
    }
}
