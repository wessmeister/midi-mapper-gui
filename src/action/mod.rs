mod executor;
mod media;

pub use executor::{execute_volume_change, execute_mute_toggle, execute_cycle_audio};
pub use media::{execute_play_pause, execute_next_track, execute_previous_track, execute_play, execute_pause};

use serde::{Deserialize, Serialize};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Action {
    None = 0,
    SystemVolume = 1,
    Mute = 2,
    PlayPause = 3,
    CycleAudioOutput = 4,
    NextTrack = 5,
    PreviousTrack = 6,
    Play = 7,
    Pause = 8,
}

impl Action {
    pub const ALL: [Action; 9] = [
        Action::None,
        Action::SystemVolume,
        Action::Mute,
        Action::PlayPause,
        Action::CycleAudioOutput,
        Action::NextTrack,
        Action::PreviousTrack,
        Action::Play,
        Action::Pause,
    ];

    pub fn label(&self) -> &str {
        match self {
            Action::None => "None",
            Action::SystemVolume => "System Volume",
            Action::Mute => "Mute Toggle",
            Action::PlayPause => "Play/Pause",
            Action::CycleAudioOutput => "Cycle Audio Output",
            Action::NextTrack => "Next Track",
            Action::PreviousTrack => "Previous Track",
            Action::Play => "Play",
            Action::Pause => "Pause",
        }
    }

    pub fn should_execute(&self, value: u8) -> bool {
        match self {
            Action::None => false,
            Action::SystemVolume => true, // Always process volume changes
            _ => value > 0,                // Others only on press
        }
    }

    pub fn execute(&self, value: u8) {
        if !self.should_execute(value) {
            return;
        }

        match self {
            Action::None => {}
            Action::SystemVolume => execute_volume_change(value),
            Action::Mute => execute_mute_toggle(),
            Action::PlayPause => execute_play_pause(),
            Action::CycleAudioOutput => execute_cycle_audio(),
            Action::NextTrack => execute_next_track(),
            Action::PreviousTrack => execute_previous_track(),
            Action::Play => execute_play(),
            Action::Pause => execute_pause(),
        }
    }
}

impl From<u32> for Action {
    fn from(v: u32) -> Self {
        match v {
            1 => Action::SystemVolume,
            2 => Action::Mute,
            3 => Action::PlayPause,
            4 => Action::CycleAudioOutput,
            5 => Action::NextTrack,
            6 => Action::PreviousTrack,
            7 => Action::Play,
            8 => Action::Pause,
            _ => Action::None,
        }
    }
}