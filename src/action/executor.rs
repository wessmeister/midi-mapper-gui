use std::process::Command;

pub fn execute_volume_change(value: u8) {
    let volume = (value as f64 / 127.0 * 100.0) as u32;
    let _ = Command::new("pactl")
        .args(&["set-sink-volume", "@DEFAULT_SINK@", &format!("{}%", volume)])
        .output();
}

pub fn execute_mute_toggle() {
    let _ = Command::new("pactl")
        .args(&["set-sink-mute", "@DEFAULT_SINK@", "toggle"])
        .output();
}

pub fn execute_cycle_audio() {
    // Get available sinks
    let Ok(output) = Command::new("pactl")
        .args(&["list", "short", "sinks"])
        .output() else { return };

    let sinks: Vec<String> = String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter_map(|line| line.split('\t').nth(1))
        .map(String::from)
        .collect();

    if sinks.is_empty() {
        return;
    }

    // Get current sink
    let Ok(current_output) = Command::new("pactl")
        .arg("get-default-sink")
        .output() else { return };

    let current = String::from_utf8_lossy(&current_output.stdout)
        .trim()
        .to_string();

    // Find next sink
    let next_idx = sinks
        .iter()
        .position(|s| s == &current)
        .map(|i| (i + 1) % sinks.len())
        .unwrap_or(0);

    // Set new default
    let _ = Command::new("pactl")
        .args(&["set-default-sink", &sinks[next_idx]])
        .output();
}