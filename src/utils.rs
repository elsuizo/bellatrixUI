use eframe::egui::{self, Ui};
use web3::types::U256;

pub fn password_ui(ui: &mut Ui, password: &mut String) -> egui::Response {
    // Generate an id for the state
    let state_id = ui.id().with("show_plaintext");

    // Get state for this widget.
    let mut show_plaintext = ui.data().get_temp::<bool>(state_id).unwrap_or(false);

    // Process ui, change a local copy of the state
    // We want TextEdit to fill entire space, and have button after that, so in that case we can
    // change direction to right_to_left.
    let result = ui.with_layout(egui::Layout::left_to_right(), |ui| {
        // Toggle the `show_plaintext` bool with a button:
        let response = ui
            .add(egui::SelectableLabel::new(show_plaintext, "👁"))
            .on_hover_text("Show/Hide Private Key");

        if response.clicked() {
            show_plaintext = !show_plaintext;
        }

        ui.add(egui::TextEdit::singleline(password).password(!show_plaintext));
        // Show the password field:
        // ui.add_sized(
        //     ui.available_size(),
        //     egui::TextEdit::singleline(password).password(!show_plaintext),
        // );
    });

    // Store the (possibly changed) state:
    ui.data().insert_temp(state_id, show_plaintext);

    result.response
}

// A wrapper that allows the more idiomatic usage pattern: `ui.add(…)`
pub fn password(password: &mut String) -> impl egui::Widget + '_ {
    move |ui: &mut egui::Ui| password_ui(ui, password)
}

pub fn validate_address_length(input: &str, desired_length: usize) -> bool {
    input.len() == desired_length
}

pub fn wei_to_eth(wei_val: U256) -> f64 {
    // ethereum does not have fractional numbers so every amount is expressed in wei, to show the
    // amount in ether this function is used ethereum no tiene numeros fraccionarios por lo que
    // toda cantidad se expresa en wei, para mostrar la cantidad en ether se utiliza esta función
    wei_val.as_u128() as f64 / 1_000_000_000_000_000_000.0f64
}
