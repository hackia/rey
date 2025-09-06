use clap::{Parser, Subcommand};
use std::process::Command;
use std::env;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Ouvre un shell DB basé sur DATABASE_URL (.env / diesel)
    Db,
}

fn main() {
    let cli = Cli::parse();
    match cli.cmd {
        Commands::Db => {
            open_db_shell();
        }
    }
}

fn open_db_shell() {
    dotenvy::dotenv().ok(); // charge ton .env si présent
    let url = env::var("DATABASE_URL")
        .expect("DATABASE_URL manquant dans l'env ou .env");

    if url.starts_with("postgres://") {
        let mut parts = url.trim_start_matches("postgres://").split('@');
        let creds = parts.next().unwrap_or("");
        let host_db = parts.next().unwrap_or("");

        let (user, db) = if creds.contains('/') {
            let mut split = creds.splitn(2, '/');
            (split.next().unwrap_or("postgres"), split.next().unwrap_or("postgres"))
        } else {
            (creds, host_db.split('/').nth(1).unwrap_or("postgres"))
        };

        let status = Command::new("psql")
            .arg("-U").arg(user)
            .arg(db)
            .status()
            .expect("échec lancement psql");
        println!("psql terminé: {status}");

    } else if url.starts_with("mysql://") {
        let mut parts = url.trim_start_matches("mysql://").split('@');
        let creds = parts.next().unwrap_or("");
        let host_db = parts.next().unwrap_or("");

        let user = creds.split(':').next().unwrap_or("root");
        let db = host_db.split('/').nth(1).unwrap_or("");

        let status = Command::new("mysql")
            .arg("-u").arg(user)
            .arg(db)
            .status()
            .expect("échec lancement mysql");
        println!("mysql terminé: {status}");

    } else if url.starts_with("sqlite://") {
        let path = url.trim_start_matches("sqlite://");
        let status = Command::new("sqlite3")
            .arg(path)
            .status()
            .expect("échec lancement sqlite3");
        println!("sqlite3 terminé: {status}");

    } else {
        eprintln!("Moteur DB non supporté dans: {url}");
    }
}
