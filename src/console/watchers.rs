use crate::console::{Console, ok_clear, views::admin::ok_command};
use notify::RecursiveMode;
use notify_debouncer_mini::new_debouncer;
use std::{
    path::Path,
    process::{Child, Command},
    sync::Arc,
    sync::atomic::{AtomicBool, Ordering},
    sync::mpsc::channel,
    time::{Duration, Instant},
};

const DEBOUNCE_MS: u64 = 700;
const COOLDOWN_MS: u64 = 1000;

fn spawn_server() -> Child {
    Command::new("cargo")
        .arg("run")
        .spawn()
        .expect("failed to start cargo run")
}

fn is_ignored_dir(path: &Path) -> bool {
    const IGNORE_DIRS: &[&str] = &["public", "dist", "target", "node_modules", ".git"];
    path.components().any(|c| {
        IGNORE_DIRS
            .iter()
            .any(|d| c.as_os_str().to_string_lossy() == *d)
    })
}

fn is_ignored_file(path: &Path) -> bool {
    if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
        if name.ends_with('~') || name == "tsconfig.tsbuildinfo" {
            return true;
        }
    }
    match path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or_default()
    {
        "css" | "js" | "map" | "swp" | "swo" | "tmp" => true,
        _ => false,
    }
}

fn is_watched_source(path: &Path) -> bool {
    match path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or_default()
    {
        "rs" | "ts" | "tsx" | "scss" | "sass" | "toml" => true,
        _ => false,
    }
}

pub fn watch() -> Result<(), Box<dyn std::error::Error>> {
    ok_clear("Watching changes...", true);
    
    let mut child = Some(spawn_server());
    let mut last_restart = Instant::now() - Duration::from_secs(10);

    let (tx, rx) = channel();
    let mut debouncer = new_debouncer(Duration::from_millis(DEBOUNCE_MS), tx)?;
    debouncer
        .watcher()
        .watch(Path::new("src"), RecursiveMode::Recursive)?;
    let _ = debouncer
        .watcher()
        .watch(Path::new("front"), RecursiveMode::Recursive);
    // Handle Ctrl+C to clean up child process
    let running = Arc::new(AtomicBool::new(true));
    {
        let running = running.clone();
        ctrlc::set_handler(move || {
            running.store(false, Ordering::SeqCst);
        })
        .expect("Error setting Ctrl-C handler");
    }

    while running.load(Ordering::SeqCst) {
        let events = match rx.recv_timeout(Duration::from_millis(200)) {
            Ok(Ok(events)) => events,
            Ok(Err(e)) => {
                eprintln!("debouncer error: {e:?}");
                continue;
            }
            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => continue,
            Err(e) => {
                eprintln!("watch error: {e:?}");
                break;
            }
        };

        let mut changed_files = vec![];
        for event in &events {
            let p = &event.path;
            if is_ignored_dir(p) || is_ignored_file(p) {
                continue;
            }
            if is_watched_source(p) {
                changed_files.push(p.display().to_string());
            }
        }
        if changed_files.is_empty() {
            continue;
        }

        if last_restart.elapsed() < Duration::from_millis(COOLDOWN_MS) {
            continue;
        }

        ok_clear(
            format!("Detected changes in: {:?}", changed_files).as_str(),
            true,
        );

        Console::compile(["ts", "scss", "rs"]);
        if let Some(mut c) = child.take() {
            if let Err(e) = c.kill() {
                eprintln!("Failed to kill child: {e}");
            }
            let _ = c.wait();
        }
        child = Some(spawn_server());
        last_restart = Instant::now();
        ok_command("reloaded", false, &mut Command::new("true"));
    }

    // Cleanup on exit
    if let Some(mut c) = child {
        let _ = c.kill();
        let _ = c.wait();
    }

    Ok(())
}
