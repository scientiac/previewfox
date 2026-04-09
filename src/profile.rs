use std::fs;
use std::path::PathBuf;

use crate::info;

// Embed asset files at compile time so the binary is self-contained
const USER_JS: &str = include_str!("assets/user.js");
const USER_CHROME_CSS: &str = include_str!("assets/userchrome.css");
const USER_CONTENT_CSS: &str = include_str!("assets/usercontent.css");
const CUSTOM_KEYS_JSON: &str = include_str!("assets/customkeys.json");

/// Returns the path to the previewer profile directory.
///
/// - Linux:   `$XDG_CACHE_HOME/ff-previewer-profile` (defaults to `~/.cache`)
/// - macOS:   `~/Library/Caches/ff-previewer-profile`
/// - Windows: `%LOCALAPPDATA%/ff-previewer-profile`
pub fn profile_dir() -> PathBuf {
    dirs::cache_dir()
        .expect("Could not determine cache directory for your platform")
        .join("ff-previewer-profile")
}

/// Creates the minimal Firefox profile if it doesn't already exist.
/// Returns the profile directory path.
pub fn create_profile(verbose: bool) -> PathBuf {
    let dir = profile_dir();
    info!(verbose, "[profile] Profile directory: {}", dir.display());

    // Skip if the profile already exists (user.js is the sentinel file)
    if dir.join("user.js").exists() {
        info!(verbose, "[profile] Profile already exists, skipping creation.");
        return dir;
    }

    info!(verbose, "[profile] No existing profile found. Creating new profile...");
    write_profile_files(&dir, verbose);
    info!(verbose, "[profile] Profile created successfully.");
    dir
}

/// Deletes the existing profile and recreates it from scratch.
/// Returns the profile directory path.
pub fn rebuild(verbose: bool) -> PathBuf {
    let dir = profile_dir();

    if dir.exists() {
        info!(verbose, "[profile] Removing existing profile at: {}", dir.display());
        fs::remove_dir_all(&dir).expect("Failed to remove existing profile directory");
        info!(verbose, "[profile] Old profile removed.");
    } else {
        info!(verbose, "[profile] No existing profile found to remove.");
    }

    info!(verbose, "[profile] Creating fresh profile...");
    write_profile_files(&dir, verbose);
    info!(verbose, "[profile] Profile rebuilt successfully.");
    dir
}

/// Checks that all essential profile files exist and prints their status.
/// Returns `true` if all files are present, `false` otherwise.
pub fn health_check(verbose: bool) -> bool {
    let dir = profile_dir();
    let files = [
        ("user.js", dir.join("user.js")),
        ("chrome/userChrome.css", dir.join("chrome").join("userChrome.css")),
        ("chrome/userContent.css", dir.join("chrome").join("userContent.css")),
        ("customKeys.json", dir.join("customKeys.json")),
    ];

    // Health check always prints (it's the whole point of the command)
    eprintln!("Profile directory: {}", dir.display());
    eprintln!();

    let mut all_ok = true;
    for (name, path) in &files {
        if path.exists() {
            eprintln!("  ✓  {name}");
            if verbose {
                eprintln!("      → {}", path.display());
            }
        } else {
            eprintln!("  ✗  {name}  (MISSING)");
            all_ok = false;
        }
    }

    eprintln!();
    if all_ok {
        eprintln!("All essential files are present.");
    } else {
        eprintln!("Some files are missing. Run with --rebuild to regenerate the profile.");
    }

    all_ok
}

/// Writes all profile files to the given directory.
fn write_profile_files(dir: &PathBuf, verbose: bool) {
    let chrome_dir = dir.join("chrome");

    info!(verbose, "[profile] Creating directory: {}", chrome_dir.display());
    fs::create_dir_all(&chrome_dir).expect("Failed to create profile chrome directory");

    info!(verbose, "[profile] Writing user.js ({} bytes)...", USER_JS.len());
    fs::write(dir.join("user.js"), USER_JS)
        .expect("Failed to write user.js");

    info!(verbose, "[profile] Writing chrome/userChrome.css ({} bytes)...", USER_CHROME_CSS.len());
    fs::write(chrome_dir.join("userChrome.css"), USER_CHROME_CSS)
        .expect("Failed to write userChrome.css");

    info!(verbose, "[profile] Writing chrome/userContent.css ({} bytes)...", USER_CONTENT_CSS.len());
    fs::write(chrome_dir.join("userContent.css"), USER_CONTENT_CSS)
        .expect("Failed to write userContent.css");

    info!(verbose, "[profile] Writing customKeys.json ({} bytes)...", CUSTOM_KEYS_JSON.len());
    fs::write(dir.join("customKeys.json"), CUSTOM_KEYS_JSON)
        .expect("Failed to write customKeys.json");
}
