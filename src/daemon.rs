use midi_mapper_linux::Action;
use midi_mapper_linux::config::load_config;
use midi_mapper_linux::midi::connect_midi_device;
use alsa::seq::{self, Seq};
use std::collections::HashMap;
use std::fs;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

static RUNNING: AtomicBool = AtomicBool::new(true);

pub fn run_daemon() -> Result<(), Box<dyn std::error::Error>> {
    // Write PID file
    let pid = std::process::id();
    let pid_path = "/tmp/midi-mapper.pid";
    fs::write(pid_path, pid.to_string())?;

    // Setup signal handler for graceful shutdown
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        eprintln!("Received shutdown signal");
        r.store(false, Ordering::SeqCst);
        RUNNING.store(false, Ordering::SeqCst);
    })?;

    eprintln!("MIDI Mapper daemon started (PID: {})", pid);

    // Connect to MIDI device
    let seq = match connect_midi_device("nanoKONTROL2") {
        Ok(s) => {
            eprintln!("Connected to nanoKONTROL2");
            s
        }
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
            return Err(e.into());
        }
    };

    // Main daemon loop
    daemon_loop(seq, running)?;

    // Cleanup
    let _ = fs::remove_file(pid_path);
    eprintln!("MIDI Mapper daemon stopped");
    Ok(())
}

fn daemon_loop(seq: Seq, running: Arc<AtomicBool>) -> Result<(), Box<dyn std::error::Error>> {
    let mut input = seq.input();
    let mut last_config_check = std::time::Instant::now();
    let mut mappings: HashMap<u8, Action> = load_config().mappings;

    while running.load(Ordering::SeqCst) {
        // Check for config updates every 5 seconds
        if last_config_check.elapsed() > Duration::from_secs(5) {
            let new_config = load_config();
            if new_config.mappings.len() != mappings.len() {
                mappings = new_config.mappings;
                eprintln!("Config reloaded: {} mappings", mappings.len());
            }
            last_config_check = std::time::Instant::now();
        }

        // Process MIDI events (non-blocking)
        if let Ok(pending) = input.event_input_pending(true) {
            if pending > 0 {
                if let Ok(ev) = input.event_input() {
                    if ev.get_type() == seq::EventType::Controller {
                        if let Some(data) = ev.get_data::<seq::EvCtrl>() {
                            let cc = data.param as u8;
                            let value = data.value as u8;

                            if let Some(&action) = mappings.get(&cc) {
                                action.execute(value);
                            }
                        }
                    }
                }
            }
        }

        // Sleep briefly to avoid CPU spin
        std::thread::sleep(Duration::from_millis(10));
    }

    Ok(())
}