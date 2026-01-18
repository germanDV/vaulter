use crate::secret::SecretError;
use std::io::Write;
use std::process::Command;

pub trait Clipboard {
    fn copy(&self, text: &str) -> Result<(), SecretError>;
}

struct WlCopy;

impl Clipboard for WlCopy {
    fn copy(&self, text: &str) -> Result<(), SecretError> {
        let mut child = Command::new("wl-copy")
            .stdin(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| {
                SecretError::ClipboardErr(format!("Failed to spawn {}: {}", "wl-copy", e))
            })?;

        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(text.as_bytes()).map_err(|e| {
                SecretError::ClipboardErr(format!("Failed to write to {}: {}", "wl-copy", e))
            })?;
        }

        child.wait().map_err(|e| {
            SecretError::ClipboardErr(format!("Failed to wait for {}: {}", "wl-copy", e))
        })?;

        Ok(())
    }
}

struct Xsel;

impl Clipboard for Xsel {
    fn copy(&self, text: &str) -> Result<(), SecretError> {
        let mut child = Command::new("xsel")
            .arg("--input")
            .arg("--clipboard")
            .stdin(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| SecretError::ClipboardErr(format!("Failed to spawn {}: {}", "xsel", e)))?;

        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(text.as_bytes()).map_err(|e| {
                SecretError::ClipboardErr(format!("Failed to write to {}: {}", "xsel", e))
            })?;
        }

        child.wait().map_err(|e| {
            SecretError::ClipboardErr(format!("Failed to wait for {}: {}", "xsel", e))
        })?;

        Ok(())
    }
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

// #[cfg(target_os = "linux")]
// fn copy_to_clipboard(text: &str) -> Result<(), SecretError> {
//     let clipboard = get_clipboard_strategy()?;
//     clipboard.copy(text)
// }

/// Factory function to select the right strategy
#[cfg(target_os = "linux")]
pub fn get_clipboard_strategy() -> Result<Box<dyn Clipboard>, SecretError> {
    if is_installed("wl-copy") {
        Ok(Box::new(WlCopy))
    } else if is_installed("xsel") {
        Ok(Box::new(Xsel))
    } else {
        Err(SecretError::ClipboardErr(
            "You need to install wl-copy or xsel".to_string(),
        ))
    }
}

#[cfg(target_os = "macos")]
pub fn get_clipboard_strategy() -> Result<Box<dyn Clipboard>, SecretError> {
    Err(SecretError::ClipboardErr(
        "No clipboard support for MacOS yet".to_string(),
    ))
}

#[cfg(target_os = "windows")]
pub fn get_clipboard_strategy() -> Result<Box<dyn Clipboard>, SecretError> {
    Err(SecretError::ClipboardErr(
        "No, why would you use Windows?".to_string(),
    ))
}
