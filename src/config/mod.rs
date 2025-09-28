mod persistence;

pub use persistence::{get_autostart_path, get_config_path, load_config, save_config, sync_autostart};

use crate::action::Action;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Config {
    pub mappings: HashMap<u8, Action>,
    pub background_enabled: bool,
    pub autostart_enabled: bool,
}