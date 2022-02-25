use eframe::egui::{self, Ui};

pub fn password_ui(ui: &mut Ui, password: &mut String) -> egui::Response {
    // Generate an id for the state
    let state_id = ui.id().with("show_plaintext");

    // Get state for this widget.
    let mut show_plaintext = ui.data().get_temp::<bool>(state_id).unwrap_or(false);

    // Process ui, change a local copy of the state
    // We want TextEdit to fill entire space, and have button after that, so in that case we can
    // change direction to right_to_left.
    let result = ui.with_layout(egui::Layout::right_to_left(), |ui| {
        // Toggle the `show_plaintext` bool with a button:
        let response = ui
            .add(egui::SelectableLabel::new(show_plaintext, "👁"))
            .on_hover_text("Show/Hide Private Key");

        if response.clicked() {
            show_plaintext = !show_plaintext;
        }

        // Show the password field:
        ui.add_sized(
            ui.available_size(),
            egui::TextEdit::singleline(password).password(!show_plaintext),
        );
    });

    // Store the (possibly changed) state:
    ui.data().insert_temp(state_id, show_plaintext);

    result.response
}

// A wrapper that allows the more idiomatic usage pattern: `ui.add(…)`
pub fn password(password: &mut String) -> impl egui::Widget + '_ {
    move |ui: &mut egui::Ui| password_ui(ui, password)
}
