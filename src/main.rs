mod clipboard;
mod crypto;
mod secret;
mod storage;

use clap::{Parser, Subcommand};
use clipboard::get_clipboard_strategy;
use crypto::Crypto;
use secret::{Secret, SecretError};
use std::env;
use storage::Store;

#[derive(Parser)]
#[command(name = "vaulter")]
#[command(about = "A CLI vault for key-value pairs", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Set (or update) a key-value pair
    #[command(alias = "s")]
    Set { key: String, val: String },
    /// Copy a value to the clipboard
    #[command(alias = "g")]
    Get { key: String },
    /// List all keys
    #[command(alias = "a")]
    All,
    /// Delete a key
    #[command(alias = "d")]
    Del { key: String },
    /// Show location of database file
    #[command(alias = "l")]
    Location,
}

fn main() {
    let cli = Cli::parse();

    let db_path = get_db_path();
    let store = Store::new(&db_path).unwrap();

    let _encryption_passphrase = get_encryption_passphrase();

    match cli.command {
        Commands::Set { key, val } => match save_secret(&store, key, val) {
            Ok(_) => println!("Secret saved successfully"),
            Err(e) => eprintln!("Error: {}", e),
        },
        Commands::Get { key } => match get_secret(&store, key) {
            Ok(_) => println!("Secret copied to clipboard"),
            Err(e) => eprintln!("Error: {}", e),
        },
        Commands::All => match list_secrets(&store) {
            Ok(keys) => {
                for key in keys {
                    println!("{}", key);
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        },
        Commands::Del { key } => match delete_secret(&store, key) {
            Ok(_) => println!("Secret deleted successfully"),
            Err(e) => eprintln!("Error: {}", e),
        },
        Commands::Location => println!("Database location: {}", get_db_path()),
    }
}

fn save_secret(store: &Store, key: String, val: String) -> Result<(), SecretError> {
    let s = Secret::new(key, val)?;
    store.save(&s)
}

fn list_secrets(store: &Store) -> Result<Vec<String>, SecretError> {
    store.list_keys()
}

fn delete_secret(store: &Store, key: String) -> Result<(), SecretError> {
    store.delete(&key)
}

fn get_secret(store: &Store, key: String) -> Result<(), SecretError> {
    let s = store.get(&key)?;
    let clipboard = get_clipboard_strategy()?;
    clipboard.copy(&s.val())
}

/// Get the path to the database file based on the OS.
fn get_db_path() -> String {
    env::var("VAULTER_DB_PATH").unwrap_or_else(|_| {
        let mut path = dirs::data_local_dir().expect("Could not determine data directory");
        path.push("vaulter");
        std::fs::create_dir_all(&path).ok(); // Create it if it doesn't exist
        path.push("vault.db");
        path.to_str().unwrap().to_string()
    })
}

/// Get the encryption passphrase from the environment variable or prompt the user for it.
/// This passphrase is used to derive an enncryption key.
fn get_encryption_passphrase() -> String {
    env::var("VAULTER_PASSPHRASE").unwrap_or_else(|_| {
        let mut passphrase = String::new();
        println!("VAULTER_PASSPHRASE not set.");
        println!("Enter encryption passphrase:");
        std::io::stdin().read_line(&mut passphrase).unwrap();
        passphrase
    })
}
