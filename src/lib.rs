pub mod action;
pub mod config;
pub mod midi;
pub mod ui;

// Re-export commonly used items
pub use action::Action;
pub use config::Config;
pub use ui::{build_ui, AppState};