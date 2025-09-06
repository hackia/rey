use std::{
    error::Error,
    fs::File,
    io::{Write, stdout},
    time::Instant,
};

// Remove Stylize to avoid method ambiguity
use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Print, Stylize},
    terminal::{Clear, ClearType, size},
};
use reqwest::blocking::get;

pub fn print_message(message: &str, status: &str, clear: bool) {
    let (width, _) = size().unwrap_or((80, 24));
    let len = width.saturating_sub(message.len() as u16 + status.len() as u16 + 5);
    let msg = format!(
        "{} {}{} {}{}{}",
        "*".green().bold(),
        message.white().bold(),
        " ".repeat(len as usize),
        "[".white().bold(),
        status.green().bold(),
        "]".white().bold()
    );
    if clear {
        execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0), Print(msg))
            .expect("Failed to print message");
        println!();
    } else {
        println!("{msg}");
    }
}

// Helper: Validate URI
pub fn validate_uri(uri: &str) -> Result<(), Box<dyn Error>> {
    if uri.is_empty() {
        Err("URI is empty".into())
    } else if !uri.starts_with("http") {
        Err("URI must start with http or https".into())
    } else {
        Ok(())
    }
}

// Helper: Validate file name
pub fn validate_file_name(file_name: &str) -> Result<(), Box<dyn Error>> {
    if file_name.is_empty() {
        Err("File name is empty".into())
    } else {
        Ok(())
    }
}

pub fn ok_command(message: &str, clear: bool, command: &mut std::process::Command) {
    if message.is_empty() {
        panic!("Message is empty");
    }
    if clear {
        execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0)).expect("Failed to clear terminal");
    }
    if command
        .status()
        .expect("Failed to execute command")
        .success()
    {
        ok_clear(message, false);
    } else {
        panic!("Command failed");
    }
}

pub fn ok_clear(message: &str, clear: bool) {
    print_message(message, "ok", clear);
}

pub fn ok_download(uri: &str, file_name: &str) -> Result<(), Box<dyn Error>> {
    validate_uri(uri)?;
    validate_file_name(file_name)?;

    let now: Instant = Instant::now();
    let response: reqwest::blocking::Response = get(uri)?;
    let content = response.bytes()?;

    let mut downloaded_file = File::create(file_name)?;
    downloaded_file.write_all(&content)?;

    let duration = now.elapsed();
    ok_clear(&format!("Downloaded {file_name} in {duration:?}"), false);
    Ok(())
}
