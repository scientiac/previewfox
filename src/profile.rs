use std::fs;
use std::path::PathBuf;

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
pub fn create_profile() -> PathBuf {
    let dir = profile_dir();

    // Skip if the profile already exists (user.js is the sentinel file)
    if dir.join("user.js").exists() {
        eprintln!("Profile already exists at: {}", dir.display());
        return dir;
    }

    write_profile_files(&dir);
    eprintln!("Profile created at: {}", dir.display());
    dir
}

/// Deletes the existing profile and recreates it from scratch.
/// Returns the profile directory path.
pub fn rebuild() -> PathBuf {
    let dir = profile_dir();

    if dir.exists() {
        fs::remove_dir_all(&dir).expect("Failed to remove existing profile directory");
        eprintln!("Removed old profile at: {}", dir.display());
    }

    write_profile_files(&dir);
    eprintln!("Profile rebuilt at: {}", dir.display());
    dir
}

/// Checks that all essential profile files exist and prints their status.
/// Returns `true` if all files are present, `false` otherwise.
pub fn health_check() -> bool {
    let dir = profile_dir();
    let files = [
        ("user.js", dir.join("user.js")),
        ("chrome/userChrome.css", dir.join("chrome").join("userChrome.css")),
        ("chrome/userContent.css", dir.join("chrome").join("userContent.css")),
        ("customKeys.json", dir.join("customKeys.json")),
    ];

    eprintln!("Profile directory: {}", dir.display());
    eprintln!();

    let mut all_ok = true;
    for (name, path) in &files {
        if path.exists() {
            eprintln!("  ✓  {name}");
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
fn write_profile_files(dir: &PathBuf) {
    let chrome_dir = dir.join("chrome");
    fs::create_dir_all(&chrome_dir).expect("Failed to create profile chrome directory");

    fs::write(dir.join("user.js"), USER_JS)
        .expect("Failed to write user.js");

    fs::write(chrome_dir.join("userChrome.css"), USER_CHROME_CSS)
        .expect("Failed to write userChrome.css");

    fs::write(chrome_dir.join("userContent.css"), USER_CONTENT_CSS)
        .expect("Failed to write userContent.css");

    fs::write(dir.join("customKeys.json"), CUSTOM_KEYS_JSON)
        .expect("Failed to write customKeys.json");
}
