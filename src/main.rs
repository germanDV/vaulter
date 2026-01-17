mod secret;

use clap::{Parser, Subcommand};
use secret::{Secret, SecretError};
use std::io::Write;
use std::process::Command;

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
    #[command(alias = "l")]
    List,
    /// Delete a key
    #[command(alias = "d")]
    Del { key: String },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Set { key, val } => match save_secret(key, val) {
            Ok(_) => println!("Secret saved successfully"),
            Err(e) => eprintln!("Error: {}", e),
        },
        Commands::Get { key } => match get_secret(key) {
            Ok(_) => println!("Secret copied to clipboard"),
            Err(e) => eprintln!("Error: {}", e),
        },
        Commands::List => match list_secrets() {
            Ok(keys) => {
                for key in keys {
                    println!("{}", key);
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        },
        Commands::Del { key } => match delete_secret(key) {
            Ok(_) => println!("Secret deleted successfully"),
            Err(e) => eprintln!("Error: {}", e),
        },
    }
}

fn save_secret(key: String, val: String) -> Result<(), SecretError> {
    let _s = Secret::new(key, val)?;
    // TODO: save secret to disk
    Ok(())
}

fn get_secret(key: String) -> Result<(), SecretError> {
    // TODO: get secret from disk
    let s = Secret::new(key, "bar".to_string())?;
    copy_to_clipboard(&s.val())
}

#[cfg(target_os = "linux")]
fn copy_to_clipboard(text: &str) -> Result<(), SecretError> {
    if is_installed("wl-copy") {
        copy_with_wlcopy(text)
    } else if is_installed("xsel") {
        copy_with_xsel(text)
    } else {
        Err(SecretError::ClipboardErr(
            "You need to install wl-copy or xsel".to_string(),
        ))
    }
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

fn is_installed(binary_name: &str) -> bool {
    Command::new("which")
        .arg(binary_name)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

fn copy_with_wlcopy(text: &str) -> Result<(), SecretError> {
    let mut child = Command::new("wl-copy")
        .stdin(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| SecretError::ClipboardErr(format!("Failed to spawn wl-copy: {}", e)))?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(text.as_bytes())
            .map_err(|e| SecretError::ClipboardErr(format!("Failed to write to wl-copy: {}", e)))?;
    }

    child
        .wait()
        .map_err(|e| SecretError::ClipboardErr(format!("Failed to wait for wl-copy: {}", e)))?;

    Ok(())
}

fn copy_with_xsel(text: &str) -> Result<(), SecretError> {
    let mut child = Command::new("xsel")
        .arg("--input")
        .arg("--clipboard")
        .stdin(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| SecretError::ClipboardErr(format!("Failed to spawn xsel: {}", e)))?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(text.as_bytes())
            .map_err(|e| SecretError::ClipboardErr(format!("Failed to write to xsel: {}", e)))?;
    }

    child
        .wait()
        .map_err(|e| SecretError::ClipboardErr(format!("Failed to wait for xsel: {}", e)))?;

    Ok(())
}

fn list_secrets() -> Result<Vec<&'static str>, SecretError> {
    let keys = vec!["foo", "bar", "baz"];
    Ok(keys)
}

fn delete_secret(_key: String) -> Result<(), SecretError> {
    // TODO: delete secret from disk
    Ok(())
}
