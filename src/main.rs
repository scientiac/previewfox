mod firefox;
mod profile;

use clap::Parser;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

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

    /// Print detailed information about each step
    #[arg(short, long)]
    verbose: bool,
}

/// Print a message only when verbose mode is enabled.
macro_rules! info {
    ($verbose:expr, $($arg:tt)*) => {
        if $verbose {
            eprintln!($($arg)*);
        }
    };
}

pub(crate) use info;

fn main() {
    let cli = Cli::parse();
    let verbose = cli.verbose;

    // --health: check and report, then exit
    if cli.health {
        info!(verbose, "[previewfox] Running health check...");
        let ok = profile::health_check(verbose);
        std::process::exit(if ok { 0 } else { 1 });
    }

    // --rebuild: nuke and recreate the profile
    let profile_dir = if cli.rebuild {
        info!(verbose, "[previewfox] Rebuild requested — removing old profile...");
        profile::rebuild(verbose)
    } else {
        info!(verbose, "[previewfox] Ensuring profile exists...");
        profile::create_profile(verbose)
    };

    // Launch Firefox
    info!(verbose, "[previewfox] Launching Firefox with URL: {}", cli.url);
    info!(verbose, "[previewfox] Profile directory: {}", profile_dir.display());
    let mut child = firefox::launch(&cli.url, &profile_dir, verbose);

    // Set up Ctrl+C / signal handler to request shutdown
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Failed to set Ctrl+C handler");

    info!(verbose, "[previewfox] Firefox is running (PID: {}). Press Ctrl+C to stop.", child.id());

    // Poll loop: check if Firefox exited or if we got a signal
    loop {
        // Check if Ctrl+C was pressed
        if !running.load(Ordering::SeqCst) {
            info!(verbose, "\n[previewfox] Interrupt received, shutting down Firefox...");
            let _ = child.kill();
            let _ = child.wait();
            info!(verbose, "[previewfox] Firefox terminated. Goodbye!");
            break;
        }

        // Check if Firefox exited on its own
        match child.try_wait() {
            Ok(Some(status)) => {
                if status.success() {
                    info!(verbose, "[previewfox] Firefox exited normally.");
                } else {
                    info!(verbose, "[previewfox] Firefox exited with status: {status}");
                }
                break;
            }
            Ok(None) => {
                // Still running, sleep briefly to avoid busy-waiting
                thread::sleep(Duration::from_millis(100));
            }
            Err(e) => {
                info!(verbose, "[previewfox] Error checking Firefox status: {e}");
                break;
            }
        }
    }
}
