use crate::config::{load_config, sync_autostart};
use crate::midi::{connect_midi_device, start_midi_thread};
use crate::ui::components::{
    create_control_bar, create_mapping_row, create_mappings_header, create_mappings_list,
    create_status_section,
};
use crate::ui::state::AppState;
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box, Orientation, Separator};
use std::sync::{Arc, Mutex};
use std::time::Duration;

const DEVICE_NAME: &str = "nanoKONTROL2";
const WINDOW_TITLE: &str = "MIDI Mapper";
const WINDOW_WIDTH: i32 = 400;
const WINDOW_HEIGHT: i32 = 500;
const WINDOW_MARGIN: i32 = 12;
const UI_POLL_MS: u64 = 50;

pub fn build_ui(app: &Application, start_in_background: bool) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title(WINDOW_TITLE)
        .default_width(WINDOW_WIDTH)
        .default_height(WINDOW_HEIGHT)
        .build();

    let main_box = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(WINDOW_MARGIN)
        .margin_top(WINDOW_MARGIN)
        .margin_bottom(WINDOW_MARGIN)
        .margin_start(WINDOW_MARGIN)
        .margin_end(WINDOW_MARGIN)
        .build();

    // Initialize state
    let config = load_config();
    sync_autostart(config.autostart_enabled);
    let state = AppState::new(config.clone());
    let seq = Arc::new(Mutex::new(None));

    // Create UI components
    let status_label = create_status_section();
    let (mappings_label, info_label) = create_mappings_header();
    let (list, scrolled) = create_mappings_list();
    let control_bar = create_control_bar(&state, &config);

    // Connect to MIDI device
    match connect_midi_device(DEVICE_NAME) {
        Ok(midi_seq) => {
            *seq.lock().unwrap() = Some(midi_seq);
            status_label.set_text(&format!("✓ Connected to {}", DEVICE_NAME));
        }
        Err(e) => {
            status_label.set_text(&format!("× {}", e));
        }
    }

    // Start MIDI thread
    let rx = start_midi_thread(Arc::clone(&seq), state.mappings.clone());

    // Load existing mappings
    let initial = state.mappings.lock().unwrap().clone();
    for (cc, _) in initial {
        let row = create_mapping_row(cc, &state);
        list.append(&row);
        state.rows.lock().unwrap().insert(cc, row);
    }

    // Handle incoming MIDI events
    let state_for_ui = state.clone();
    let list_clone = list.clone();
    gtk4::glib::timeout_add_local(Duration::from_millis(UI_POLL_MS), move || {
        if let Ok((cc, _)) = rx.try_recv() {
            let mut rows = state_for_ui.rows.lock().unwrap();
            if !rows.contains_key(&cc) {
                let row = create_mapping_row(cc, &state_for_ui);
                list_clone.append(&row);
                rows.insert(cc, row);
            }
        }
        gtk4::glib::ControlFlow::Continue
    });

    // Setup close handler
    let state_for_close = state.clone();
    let app_for_close = app.clone();
    window.connect_close_request(move |window| {
        if state_for_close.is_background_enabled() {
            window.set_visible(false);
            gtk4::glib::Propagation::Stop
        } else {
            app_for_close.quit();
            gtk4::glib::Propagation::Proceed
        }
    });

    // Assemble UI
    main_box.append(&status_label);
    main_box.append(&Separator::new(Orientation::Horizontal));
    main_box.append(&mappings_label);
    main_box.append(&info_label);
    main_box.append(&scrolled);
    main_box.append(&Separator::new(Orientation::Horizontal));
    main_box.append(&control_bar);

    window.set_child(Some(&main_box));

    // Present window
    if start_in_background && state.is_background_enabled() {
        window.present();
        window.minimize();
    } else {
        window.present();
    }
}