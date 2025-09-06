use crate::console::views::admin::{ok_clear, ok_command};

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
