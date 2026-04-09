use std::path::Path;
use std::process::{Child, Command, Stdio};

use crate::info;

/// Locates the Firefox binary for the current platform.
///
/// - Linux:   `firefox` on PATH
/// - macOS:   `/Applications/Firefox.app/Contents/MacOS/firefox`, fallback to PATH
/// - Windows: `C:\Program Files\Mozilla Firefox\firefox.exe`, fallback to PATH
pub fn find_firefox(verbose: bool) -> String {
    info!(verbose, "[firefox] Detecting Firefox binary for current platform...");

    #[cfg(target_os = "macos")]
    {
        let app_path = "/Applications/Firefox.app/Contents/MacOS/firefox";
        info!(verbose, "[firefox] Checking macOS app bundle: {app_path}");
        if Path::new(app_path).exists() {
            info!(verbose, "[firefox] Found Firefox at: {app_path}");
            return app_path.to_string();
        }
        info!(verbose, "[firefox] macOS app bundle not found, falling back to PATH.");
    }

    #[cfg(target_os = "windows")]
    {
        let program_files = std::env::var("ProgramFiles").unwrap_or_else(|_| r"C:\Program Files".to_string());
        let exe_path = format!(r"{}\Mozilla Firefox\firefox.exe", program_files);
        info!(verbose, "[firefox] Checking: {exe_path}");
        if Path::new(&exe_path).exists() {
            info!(verbose, "[firefox] Found Firefox at: {exe_path}");
            return exe_path;
        }

        let program_files_x86 =
            std::env::var("ProgramFiles(x86)").unwrap_or_else(|_| r"C:\Program Files (x86)".to_string());
        let exe_path_x86 = format!(r"{}\Mozilla Firefox\firefox.exe", program_files_x86);
        info!(verbose, "[firefox] Checking: {exe_path_x86}");
        if Path::new(&exe_path_x86).exists() {
            info!(verbose, "[firefox] Found Firefox at: {exe_path_x86}");
            return exe_path_x86;
        }
        info!(verbose, "[firefox] Windows paths not found, falling back to PATH.");
    }

    // Fallback: assume `firefox` is on PATH (works on Linux, and as fallback everywhere)
    info!(verbose, "[firefox] Using 'firefox' from PATH.");
    "firefox".to_string()
}

/// Launches Firefox with the previewer profile and returns the child process handle.
pub fn launch(url: &str, profile_dir: &Path, verbose: bool) -> Child {
    let firefox = find_firefox(verbose);

    info!(verbose, "[firefox] Spawning: {firefox} --new-window --profile {} --no-remote {url}", profile_dir.display());

    Command::new(&firefox)
        .arg("--new-window")
        .arg("--profile")
        .arg(profile_dir)
        .arg("--no-remote")
        .arg(url)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap_or_else(|e| {
            eprintln!("Failed to launch Firefox (tried: {firefox})");
            eprintln!("Error: {e}");
            eprintln!();
            eprintln!("Make sure Firefox is installed and accessible on your PATH.");
            std::process::exit(1);
        })
}
