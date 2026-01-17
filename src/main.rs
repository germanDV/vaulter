mod clipboard;
mod secret;
mod storage;

use clap::{Parser, Subcommand};
use clipboard::get_clipboard_strategy;
use secret::{Secret, SecretError};
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
    /// Set a key-value pair
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
}

fn main() {
    let cli = Cli::parse();

    let db_path = "vaulter.sqlite";
    let store = Store::new(db_path).unwrap();

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
    copy_to_clipboard(&s.val())
}

#[cfg(target_os = "linux")]
fn copy_to_clipboard(text: &str) -> Result<(), SecretError> {
    let clipboard = get_clipboard_strategy()?;
    clipboard.copy(text)
}

#[cfg(target_os = "macos")]
fn copy_to_clipboard(text: &str) -> Result<(), SecretError> {
    Err(SecretError::ClipboardErr(
        "No clipboard support for MacOS yet".to_string(),
    ))
}

#[cfg(target_os = "windows")]
fn copy_to_clipboard(text: &str) -> Result<(), SecretError> {
    Err(SecretError::ClipboardErr("Why?".to_string()))
}
