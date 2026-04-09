mod firefox;
mod profile;

use clap::Parser;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[derive(Parser)]
#[command(
    name = "previewfox",
    about = "Launch Firefox with a minimal, chrome-less UI for live previews.",
    long_about = "PreviewFox strips away all Firefox UI (tabs, toolbar, sidebar, bookmarks) \
                  and opens a URL in a clean content-only window. Perfect for live-previewing \
                  web projects, kiosk displays, or embedded browser views.",
    version
)]
struct Cli {
    /// URL to open in the preview window (defaults to about:logo)
    #[arg(default_value = "about:logo")]
    url: String,

    /// Force-rebuild the profile from scratch (deletes and regenerates all profile files)
    #[arg(short, long)]
    rebuild: bool,

    /// Check that all essential profile files exist and report their status
    #[arg(short = 'H', long)]
    health: bool,
}

fn main() {
    let cli = Cli::parse();

    // --health: check and report, then exit
    if cli.health {
        let ok = profile::health_check();
        std::process::exit(if ok { 0 } else { 1 });
    }

    // --rebuild: nuke and recreate the profile
    let profile_dir = if cli.rebuild {
        profile::rebuild()
    } else {
        profile::create_profile()
    };

    // Launch Firefox
    let mut child = firefox::launch(&cli.url, &profile_dir);
    let child_id = child.id();

    // Set up Ctrl+C / signal handler to kill Firefox when we exit
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        eprintln!("\nInterrupt received, shutting down Firefox...");
        r.store(false, Ordering::SeqCst);

        // Kill the Firefox process
        #[cfg(unix)]
        {
            // Send SIGTERM on Unix
            unsafe {
                libc::kill(child_id as i32, libc::SIGTERM);
            }
        }

        #[cfg(not(unix))]
        {
            // On Windows, we can't easily send signals — use taskkill
            let _ = std::process::Command::new("taskkill")
                .args(["/PID", &child_id.to_string(), "/T", "/F"])
                .output();
        }
    })
    .expect("Failed to set Ctrl+C handler");

    // Wait for Firefox to exit
    match child.wait() {
        Ok(status) => {
            if !status.success() && running.load(Ordering::SeqCst) {
                // Only report non-zero exit if we didn't trigger the kill ourselves
                eprintln!("Firefox exited with status: {status}");
            }
        }
        Err(e) => {
            eprintln!("Error waiting for Firefox: {e}");
        }
    }
}
