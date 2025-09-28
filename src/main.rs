mod daemon;

use gtk4::prelude::*;
use gtk4::Application;
use midi_mapper_linux::build_ui;

const APP_ID: &str = "com.simple.midimapper";

fn main() {
    // Check for daemon mode first (before GTK touches args)
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"--daemon".to_string()) {
        // Run in daemon mode - no GTK
        if let Err(e) = daemon::run_daemon() {
            eprintln!("Daemon error: {}", e);
            std::process::exit(1);
        }
        return;
    }

    // Check for --background flag before GTK processes args
    let start_in_background = args.contains(&"--background".to_string());

    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(move |app| {
        build_ui(app, start_in_background);
    });

    app.run();
}