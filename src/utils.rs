use eframe::egui::{self, Color32, Hyperlink, Label, RichText, Separator, TextStyle, Ui};
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

pub fn render_monospaced_label(ui: &mut Ui, text: &str) {
    ui.add(Label::new(
        RichText::new(text)
            .color(Color32::WHITE)
            .heading()
            .text_style(TextStyle::Monospace)
            .size(20.0),
    ));
}

pub fn render_hyperlink(ui: &mut Ui, url: &str, label: &str) {
    ui.add(Hyperlink::from_label_and_url(label, url));
}

pub fn render_separator(ui: &mut Ui, amount: f32, spacing: f32) {
    ui.add_space(amount);
    let sep = Separator::default().spacing(spacing);
    ui.add(sep);
}

/// Here is the same code again, but a bit more compact:
#[allow(dead_code)]
pub fn toggle_ui_compact(ui: &mut egui::Ui, on: &mut bool) -> egui::Response {
    let desired_size = ui.spacing().interact_size.y * egui::vec2(2.0, 1.0);
    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
    if response.clicked() {
        *on = !*on;
        response.mark_changed();
    }
    response.widget_info(|| egui::WidgetInfo::selected(egui::WidgetType::Checkbox, *on, ""));

    if ui.is_rect_visible(rect) {
        let how_on = ui.ctx().animate_bool(response.id, *on);
        let visuals = ui.style().interact_selectable(&response, *on);
        let rect = rect.expand(visuals.expansion);
        let radius = 0.5 * rect.height();
        ui.painter()
            .rect(rect, radius, visuals.bg_fill, visuals.bg_stroke);
        let circle_x = egui::lerp((rect.left() + radius)..=(rect.right() - radius), how_on);
        let center = egui::pos2(circle_x, rect.center().y);
        ui.painter()
            .circle(center, 0.75 * radius, visuals.bg_fill, visuals.fg_stroke);
    }

    response
}

// A wrapper that allows the more idiomatic usage pattern: `ui.add(toggle(&mut my_bool))`
/// iOS-style toggle switch.
///
/// ## Example:
/// ``` ignore
/// ui.add(toggle(&mut my_bool));
/// ```
pub fn toggle(on: &mut bool) -> impl egui::Widget + '_ {
    move |ui: &mut egui::Ui| toggle_ui_compact(ui, on)
}
