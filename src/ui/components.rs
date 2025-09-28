use crate::action::Action;
use crate::config::{load_config, save_config, sync_autostart, Config};
use crate::ui::state::AppState;
use gtk4::prelude::*;
use gtk4::{Box, Button, CheckButton, ComboBoxText, Label, ListBox, ListBoxRow, Orientation, ScrolledWindow};

const UI_SPACING: i32 = 8;

pub fn create_status_section() -> Label {
    let label = Label::new(Some("Connecting..."));
    label.add_css_class("dim-label");
    label
}

pub fn create_mappings_header() -> (Label, Label) {
    let title = Label::new(Some("Active Mappings"));
    title.add_css_class("heading");
    title.set_halign(gtk4::Align::Start);

    let info = Label::new(Some("(Move any MIDI control to detect it)"));
    info.add_css_class("dim-label");
    info.set_halign(gtk4::Align::Start);

    (title, info)
}

pub fn create_mappings_list() -> (ListBox, ScrolledWindow) {
    let list = ListBox::new();
    list.set_selection_mode(gtk4::SelectionMode::None);

    // Sort by CC number using property
    list.set_sort_func(|row1, row2| {
        let cc1: String = row1.property("name");
        let cc2: String = row2.property("name");
        cc1.cmp(&cc2).into()
    });

    let scrolled = ScrolledWindow::builder().vexpand(true).build();
    scrolled.set_child(Some(&list));

    (list, scrolled)
}

pub fn create_mapping_row(cc: u8, state: &AppState) -> ListBoxRow {
    let row = ListBoxRow::new();
    row.set_property("name", format!("cc-{:03}", cc));

    let hbox = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(UI_SPACING)
        .margin_top(4)
        .margin_bottom(4)
        .margin_start(4)
        .margin_end(4)
        .build();

    // CC label
    let label = Label::new(Some(&format!("CC{:02}", cc)));
    label.set_width_chars(5);

    // Arrow
    let arrow = Label::new(Some("→"));
    arrow.add_css_class("dim-label");

    // Action dropdown
    let combo = ComboBoxText::new();
    for action in &Action::ALL {
        combo.append_text(action.label());
    }

    let current = state
        .mappings
        .lock()
        .unwrap()
        .get(&cc)
        .copied()
        .unwrap_or(Action::None);
    combo.set_active(Some(current as u32));

    // Handle dropdown changes
    let state_for_save = state.clone();
    combo.connect_changed(move |combo| {
        if let Some(idx) = combo.active() {
            state_for_save.update_mapping(cc, Action::from(idx));
        }
    });

    // Clear button
    let clear_btn = Button::with_label("×");
    clear_btn.add_css_class("flat");
    clear_btn.add_css_class("circular");

    let row_weak = row.downgrade();
    let rows_clone = state.rows.clone();
    let state_for_clear = state.clone();

    clear_btn.connect_clicked(move |_| {
        state_for_clear.remove_mapping(cc);
        rows_clone.lock().unwrap().remove(&cc);

        if let Some(row) = row_weak.upgrade() {
            if let Some(parent) = row.parent() {
                if let Some(list) = parent.downcast_ref::<ListBox>() {
                    list.remove(&row);
                }
            }
        }
    });

    hbox.append(&label);
    hbox.append(&arrow);
    hbox.append(&combo);
    hbox.append(&clear_btn);
    row.set_child(Some(&hbox));

    row
}

pub fn create_control_bar(state: &AppState, config: &Config) -> Box {
    let control_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(16)
        .build();

    let background_check = CheckButton::with_label("Keep running in background");
    background_check.set_active(state.is_background_enabled());
    background_check.set_tooltip_text(Some("Keep MIDI mappings active when window is closed"));

    let autostart_check = CheckButton::with_label("Start on login");
    autostart_check.set_active(config.autostart_enabled);
    autostart_check.set_tooltip_text(Some("Automatically start when you log in"));

    // Handle background checkbox changes
    let state_clone = state.clone();
    background_check.connect_toggled(move |check| {
        state_clone.set_background_enabled(check.is_active());
    });

    // Handle autostart checkbox changes
    autostart_check.connect_toggled(move |check| {
        let mut config = load_config();
        config.autostart_enabled = check.is_active();
        save_config(&config);
        sync_autostart(check.is_active());
    });

    control_box.append(&background_check);
    control_box.append(&autostart_check);
    control_box
}