//! DeFiveK Desktop — Phase 3
//! Modular Linux Desktop Shell for Debian GNU/Linux 13 (Trixie) ARM64
//! Supporting Windows 7, OS X Yosemite, Android 10, and Aurora Modes
//! Featuring Premium Design Tokens, Vector Iconography & Wallpaper Personalization.

mod app;
mod config;
mod modes;
mod theme;

use app::DeFiveKApp;
use config::DeFiveKConfig;

fn main() {
    println!("=======================================================");
    println!("  DeFiveK Desktop Shell — Phase 3 Visual Excellence");
    println!("  Target: Debian 13 (Trixie) ARM64 / X11 / VirGL");
    println!("  Modes: Windows 7 | OS X Yosemite | Android 10 | Aurora");
    println!("=======================================================");

    // Parse command line arguments
    let mut fullscreen = false;
    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "--fullscreen" | "-f" => fullscreen = true,
            "--help" | "-h" => {
                println!("DeFiveK Desktop — Modular Linux Desktop Shell");
                println!("Usage: defivek [OPTIONS]");
                println!();
                println!("Options:");
                println!("  -f, --fullscreen    Launch the desktop shell in fullscreen mode");
                println!("  -h, --help          Show this help message");
                return;
            }
            _ => {}
        }
    }

    // Determine and announce configuration path
    let config_path = DeFiveKConfig::config_path();
    println!("[DeFiveK Shell] Configuration target: {:?}", config_path);

    // Load persisted configuration or defaults
    let config = DeFiveKConfig::load();
    println!(
        "[DeFiveK Shell] Initial active mode: {} (Dark mode: {}, Fullscreen: {})",
        config.mode.title(),
        config.dark_mode,
        fullscreen
    );

    // Initialize application runtime
    let app = match DeFiveKApp::new(config, fullscreen) {
        Ok(app) => app,
        Err(err) => {
            eprintln!("[DeFiveK Shell] Fatal error initializing application: {}", err);
            std::process::exit(1);
        }
    };

    // Run Slint event loop
    if let Err(err) = app.run() {
        eprintln!("[DeFiveK Shell] Application runtime error: {}", err);
        std::process::exit(1);
    }
}
