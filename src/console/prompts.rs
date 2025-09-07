use crate::console::views::admin::{ok_clear, ok_command, ok_download};
use std::io::Error;

pub struct Question;

impl Question {
    /// Prompts for confirmation (yes/no).
    pub fn confirm(
        prompt: &str,
        default: Option<&str>,
        help: Option<&str>,
    ) -> Result<bool, inquire::error::InquireError> {
        inquire::Confirm::new(prompt)
            .with_default(
                default
                    .map(|d| {
                        let d = d.to_lowercase();
                        d == "y" || d == "1" || d == "yes"
                    })
                    .unwrap_or(false),
            )
            .with_help_message(help.map_or("", |h| h))
            .prompt()
    }

    /// Prompts for text input.
    pub fn ask(
        prompt: &str,
        default: Option<&str>,
        help: Option<&str>,
    ) -> Result<String, inquire::error::InquireError> {
        let mut text_prompt = inquire::Text::new(prompt);
        if let Some(default) = default {
            text_prompt = text_prompt.with_default(default);
        }
        text_prompt.with_help_message(help.unwrap_or("")).prompt()
    }

    /// Prompts for selection from values.
    pub fn select(
        prompt: &str,
        values: &[String],
        help: Option<&str>,
    ) -> Result<String, inquire::error::InquireError> {
        inquire::Select::new(prompt, values.to_vec())
            .with_help_message(help.unwrap_or(""))
            .prompt()
    }
    /// Prompts for multiple selection from values.
    pub fn multiple_select(
        prompt: &str,
        values: &[String],
        help: Option<&str>,
    ) -> Result<Vec<String>, inquire::error::InquireError> {
        inquire::MultiSelect::new(prompt, values.to_vec())
            .with_help_message(help.unwrap_or(""))
            .prompt()
    }

    /// Prompts for password input.
    pub fn pass(prompt: &str, help: Option<&str>) -> Result<(), inquire::error::InquireError> {
        inquire::Password::new(prompt)
            .with_help_message(help.unwrap_or(""))
            .prompt()
            .map(|_| ())
    }
    /// Prompts for multi-line text input (opens editor).
    pub fn editor(
        prompt: &str,
        help: Option<&str>,
    ) -> Result<String, inquire::error::InquireError> {
        inquire::Editor::new(prompt)
            .with_help_message(help.unwrap_or(""))
            .prompt()
    }
}

pub enum Licenses {
    MIT,
    Apache2,
    GPL3,
    BSD3,
    Unlicense,
    MPL2,
    AGPL3,
    LGPL3,
    Artistic2,
    CC0,
    EUPL1_2,
    Zlib,
    None,
}
impl Licenses {
    pub fn variants() -> Vec<String> {
        vec![
            "MIT".to_string(),
            "Apache-2.0".to_string(),
            "GPL-3.0".to_string(),
            "BSD-3-Clause".to_string(),
            "Unlicense".to_string(),
            "MPL-2.0".to_string(),
            "AGPL-3.0".to_string(),
            "LGPL-3.0".to_string(),
            "Artistic-2.0".to_string(),
            "CC0-1.0".to_string(),
            "EUPL-1.2".to_string(),
            "Zlib".to_string(),
            "None".to_string(),
        ]
    }
    pub fn from_str(s: &str) -> Self {
        match s {
            "MIT" => Licenses::MIT,
            "Apache-2.0" => Licenses::Apache2,
            "GPL-3.0" => Licenses::GPL3,
            "BSD-3-Clause" => Licenses::BSD3,
            "Unlicense" => Licenses::Unlicense,
            "MPL-2.0" => Licenses::MPL2,
            "AGPL-3.0" => Licenses::AGPL3,
            "LGPL-3.0" => Licenses::LGPL3,
            "Artistic-2.0" => Licenses::Artistic2,
            "CC0-1.0" => Licenses::CC0,
            "EUPL-1.2" => Licenses::EUPL1_2,
            "Zlib" => Licenses::Zlib,
            _ => Licenses::None,
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            Licenses::MIT => "MIT",
            Licenses::Apache2 => "Apache-2.0",
            Licenses::GPL3 => "GPL-3.0",
            Licenses::BSD3 => "BSD-3-Clause",
            Licenses::Unlicense => "Unlicense",
            Licenses::MPL2 => "MPL-2.0",
            Licenses::AGPL3 => "AGPL-3.0",
            Licenses::LGPL3 => "LGPL-3.0",
            Licenses::Artistic2 => "Artistic-2.0",
            Licenses::CC0 => "CC0-1.0",
            Licenses::EUPL1_2 => "EUPL-1.2",
            Licenses::Zlib => "Zlib",
            Licenses::None => "None",
        }
    }
    pub fn download(&self) -> Result<(), Error> {
        if let Some(uri) = match self {
            Licenses::MIT => {
                Some("https://raw.githubusercontent.com/hackiado/licenses/refs/heads/main/MIT")
            }
            Licenses::Apache2 => Some(
                "https://raw.githubusercontent.com/hackiado/licenses/refs/heads/main/Apache-2.0",
            ),
            Licenses::GPL3 => {
                Some("https://raw.githubusercontent.com/hackiado/licenses/refs/heads/main/GPL-3.0")
            }
            Licenses::BSD3 => Some(
                "https://raw.githubusercontent.com/hackiado/licenses/refs/heads/main/BSD-3-Clause",
            ),
            Licenses::Unlicense => Some(
                "https://raw.githubusercontent.com/hackiado/licenses/refs/heads/main/Unlicense",
            ),
            Licenses::MPL2 => {
                Some("https://raw.githubusercontent.com/hackiado/licenses/refs/heads/main/MPL-2.0")
            }
            Licenses::AGPL3 => {
                Some("https://raw.githubusercontent.com/hackiado/licenses/refs/heads/main/AGPL_V3")
            }
            Licenses::LGPL3 => {
                Some("https://raw.githubusercontent.com/hackiado/licenses/refs/heads/main/LGPL-3.0")
            }
            Licenses::Artistic2 => Some(
                "https://raw.githubusercontent.com/hackiado/licenses/refs/heads/main/Artistic-2.0",
            ),
            Licenses::CC0 => {
                Some("https://raw.githubusercontent.com/hackiado/licenses/refs/heads/main/CC0-1.0")
            }
            Licenses::EUPL1_2 => {
                Some("https://raw.githubusercontent.com/hackiado/licenses/refs/heads/main/EUPL-1.2")
            }
            Licenses::Zlib => {
                Some("https://raw.githubusercontent.com/hackiado/licenses/refs/heads/main/Zlib")
            }
            Licenses::None => None,
        } {
            if ok_download(uri, "LICENSE").is_ok() {
                ok_clear("License file downloaded.", false);
                Ok(())
            } else {
                ok_clear("Failed to download license file.", false);
                Err(Error::other("Failed to download license file."))
            }
        } else {
            ok_clear("No license selected.", false);
            Ok(())
        }
    }
}
pub struct Commit;

impl Commit {
    /// Prompts for commit message.
    pub fn message(
        prompt: &str,
        default: Option<&str>,
        help: Option<&str>,
    ) -> Result<String, inquire::error::InquireError> {
        Question::editor(prompt, help).or_else(|_| {
            Question::ask(
                prompt,
                default.or(Some("")),
                Some("Commit message cannot be empty."),
            )
            .and_then(|msg| {
                if msg.trim().is_empty() {
                    Err(inquire::error::InquireError::OperationCanceled)
                } else {
                    Ok(msg)
                }
            })
        })
    }

    /// Executes `hg commit` and `hg push` commands and shows status messages.
    pub fn commit_and_push() -> bool {
        ok_command(
            "committing changes",
            false,
            std::process::Command::new("hg")
                .arg("commit")
                .arg("-m")
                .arg(
                    Question::editor("Commit message:", Some("enter commit message"))
                        .unwrap_or_else(|_| panic!("Commit message is required.")),
                ),
        );
        ok_command(
            "sending push request",
            false,
            std::process::Command::new("hg").arg("push"),
        );
        true
    }
    /// Executes `hg push` command and shows status message.
    pub fn push() -> bool {
        ok_command(
            "sending push request",
            false,
            std::process::Command::new("hg").arg("push"),
        );
        true
    }

    /// Prompts for confirmation to send the commit.
    pub fn send() -> Result<bool, inquire::error::InquireError> {
        Question::confirm(
            "Send commit?",
            Some("y"),
            Some("Send the commit to the remote repository."),
        )
    }
    /// Displays the `hg diff` output and prompts to add changes.
    pub fn diff() {
        if std::process::Command::new("hg")
            .arg("diff")
            .status()
            .expect("Failed to execute hg diff")
            .success()
        {
            if Question::confirm("add changes to commit?", Some("y"), None).unwrap_or(false) {
                if Question::confirm("add changes to commit?", Some("y"), None).unwrap_or(false) {
                    std::process::Command::new("hg")
                        .arg("add")
                        .arg(".")
                        .status()
                        .expect("Failed to execute hg add");
                    ok_clear("Changes added to commit.", false);
                } else {
                    ok_clear("Changes not added to commit.", false);
                }
            } else {
                ok_clear("Failed to display diff.", false);
            }
        }
    }
    /// Prompts for confirmation to view changes.
    pub fn get_changes() -> Result<bool, inquire::error::InquireError> {
        Question::confirm(
            "View changes?",
            Some("n"),
            Some("View the changes to be committed."),
        )
    }
    pub fn abort() -> bool {
        if Question::confirm(
            "Abort commit?",
            Some("n"),
            Some("Abort the current commit operation."),
        )
        .unwrap_or(false)
        {
            ok_clear("Commit aborted.", false);
            true
        } else {
            false
        }
    }
}
