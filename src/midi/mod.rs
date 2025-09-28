mod device;
mod processor;

pub use device::connect_midi_device;
pub use processor::start_midi_thread;

// Re-export ALSA types that are used in the public API
pub use alsa::seq::Seq;