use crate::action::Action;
use crate::config::{load_config, save_config, Config};
use gtk4::ListBoxRow;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub type MappingStore = Arc<Mutex<HashMap<u8, Action>>>;
pub type RowStore = Arc<Mutex<HashMap<u8, ListBoxRow>>>;

pub struct AppState {
    pub mappings: MappingStore,
    pub background_enabled: Arc<Mutex<bool>>,
    pub rows: RowStore,
}

impl AppState {
    pub fn new(config: Config) -> Self {
        Self {
            mappings: Arc::new(Mutex::new(config.mappings)),
            background_enabled: Arc::new(Mutex::new(config.background_enabled)),
            rows: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn save(&self) {
        let mappings = self.mappings.lock().unwrap().clone();
        let background = *self.background_enabled.lock().unwrap();

        // Load existing config to preserve autostart_enabled
        let mut config = load_config();
        config.mappings = mappings;
        config.background_enabled = background;
        save_config(&config);
    }

    pub fn update_mapping(&self, cc: u8, action: Action) {
        let mut mappings = self.mappings.lock().unwrap();
        if action == Action::None {
            mappings.remove(&cc);
        } else {
            mappings.insert(cc, action);
        }
        drop(mappings); // Explicit drop before save
        self.save();
    }

    pub fn remove_mapping(&self, cc: u8) {
        self.mappings.lock().unwrap().remove(&cc);
        self.save();
    }

    pub fn set_background_enabled(&self, enabled: bool) {
        *self.background_enabled.lock().unwrap() = enabled;
        self.save();
    }

    pub fn is_background_enabled(&self) -> bool {
        *self.background_enabled.lock().unwrap()
    }
}

// Clone implementation for UI callbacks
impl Clone for AppState {
    fn clone(&self) -> Self {
        Self {
            mappings: self.mappings.clone(),
            background_enabled: self.background_enabled.clone(),
            rows: self.rows.clone(),
        }
    }
}