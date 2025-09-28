use super::Config;
use std::fs;
use std::path::PathBuf;

pub fn get_config_path() -> PathBuf {
    let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("midi-mapper");
    path.push("config.json");
    path
}

pub fn load_config() -> Config {
    let path = get_config_path();
    if let Ok(content) = fs::read_to_string(&path) {
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        Config {
            background_enabled: true,
            ..Default::default()
        }
    }
}

pub fn save_config(config: &Config) {
    let path = get_config_path();
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    if let Ok(json) = serde_json::to_string_pretty(config) {
        let _ = fs::write(&path, json);
    }
}

pub fn get_autostart_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("autostart")
        .join("midi-mapper-linux.desktop")
}

pub fn sync_autostart(enabled: bool) {
    let path = get_autostart_path();
    if enabled {
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let desktop_entry = "[Desktop Entry]\n\
            Type=Application\n\
            Name=MIDI Mapper\n\
            Exec=midi-mapper-linux --background\n\
            Icon=midi-mapper-linux\n\
            Terminal=false\n\
            Categories=Audio;AudioVideo;\n\
            X-GNOME-Autostart-enabled=true";
        let _ = fs::write(path, desktop_entry);
    } else {
        let _ = fs::remove_file(path);
    }
}