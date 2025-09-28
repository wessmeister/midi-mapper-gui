use std::process::Command;

// Helper function to send media commands via qdbus to all MPRIS players
fn execute_media_command(action: &str) {
    // Get all MPRIS players
    if let Ok(output) = Command::new("qdbus").output() {
        let players: Vec<String> = String::from_utf8_lossy(&output.stdout)
            .lines()
            .filter(|line| line.contains("org.mpris.MediaPlayer2"))
            .map(|line| line.trim().to_string())
            .collect();

        // Send command to all players
        for player in players {
            let _ = Command::new("qdbus")
                .args(&[
                    &player,
                    "/org/mpris/MediaPlayer2",
                    &format!("org.mpris.MediaPlayer2.Player.{}", action),
                ])
                .output();
        }
    }
}

pub fn execute_play_pause() {
    execute_media_command("PlayPause");
}

pub fn execute_next_track() {
    execute_media_command("Next");
}

pub fn execute_previous_track() {
    execute_media_command("Previous");
}

pub fn execute_play() {
    execute_media_command("Play");
}

pub fn execute_pause() {
    execute_media_command("Pause");
}