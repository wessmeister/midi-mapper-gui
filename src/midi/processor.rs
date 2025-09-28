use crate::action::Action;
use alsa::seq::{self, Seq};
use std::collections::HashMap;
use std::sync::mpsc::{self, Receiver};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

const MIDI_POLL_MS: u64 = 10;

pub type MappingStore = Arc<Mutex<HashMap<u8, Action>>>;

pub fn start_midi_thread(
    seq: Arc<Mutex<Option<Seq>>>,
    mappings: MappingStore,
) -> Receiver<(u8, u8)> {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || loop {
        if let Some(seq) = seq.lock().unwrap().as_ref() {
            let mut input = seq.input();
            if let Ok(pending) = input.event_input_pending(true) {
                if pending > 0 {
                    if let Ok(ev) = input.event_input() {
                        if ev.get_type() == seq::EventType::Controller {
                            if let Some(data) = ev.get_data::<seq::EvCtrl>() {
                                let cc = data.param as u8;
                                let value = data.value as u8;

                                // Send to UI
                                let _ = tx.send((cc, value));

                                // Execute action
                                if let Some(&action) = mappings.lock().unwrap().get(&cc) {
                                    action.execute(value);
                                }
                            }
                        }
                    }
                }
            }
        }
        thread::sleep(Duration::from_millis(MIDI_POLL_MS));
    });

    rx
}