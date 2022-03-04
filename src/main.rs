mod bellatrix;
mod utils;

use eframe::epi;

use crate::bellatrix::Bellatrix;
use eframe::egui::{
    self, Color32, Context, Hyperlink, Label, RichText, Separator, TextStyle, TopBottomPanel, Ui,
    Visuals,
};

pub const PADDING: f32 = 5.0;
const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
const CYAN: Color32 = Color32::from_rgb(0, 255, 255);

fn render_footer(ctx: &Context) {
    TopBottomPanel::bottom("footer").show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(10.);
            render_monospaced_label(ui, "Made with ♥ By Cau1dr0n 1ake");
            render_monospaced_label(ui, "Donate: 0x43aF68DcC19Bbce20F3354F31Bc1159f651643aE");
            render_hyperlink(ui, "https://domain.com", "Buy API KEY");
            ui.add_space(10.);
        })
    });
}

fn render_monospaced_label(ui: &mut Ui, text: &str) {
    ui.add(Label::new(
        RichText::new(text)
            .color(WHITE)
            .heading()
            .text_style(TextStyle::Monospace)
            .size(30.0),
    ));
}

fn render_hyperlink(ui: &mut Ui, url: &str, label: &str) {
    ui.add(Hyperlink::from_label_and_url(label, url));
}

fn render_separator(ui: &mut Ui, amount: f32, spacing: f32) {
    ui.add_space(amount);
    let sep = Separator::default().spacing(spacing);
    ui.add(sep);
}

fn render_header(ui: &mut Ui) {
    ui.vertical_centered(|ui| {
        ui.add(Label::new(
            RichText::new("Account Info").color(WHITE).heading(),
        ));

        ui.label("");

        ui.vertical_centered(|ui2| {
            ui2.label("0xa2F9...1b3E");
            ui2.label("Balance: 1.233");
        });
    });

    render_separator(ui, PADDING, 20.);
}

impl epi::App for Bellatrix {
    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        // TODO(elsuizo:2022-02-26): add more Options for this panel
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            render_header(ui);

            render_footer(ctx);

            self.render_wallet_section(ui);

            self.render_addres_section(ui);

            self.render_middle_section(ui);

            self.render_token_wallet_section(ui);

            self.render_tracking_information_section(ui);

            egui::warn_if_debug_build(ui);
        });
    }

    fn setup(
        &mut self,
        ctx: &egui::Context,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        // enable dark mode
        ctx.set_visuals(Visuals::dark());

        self.configure_fonts(ctx);

        // TODO(elsuizo:2022-03-04): parece que esto no anda ...
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }
    }

    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    fn name(&self) -> &str {
        "BellatrixUI app"
    }
}

fn main() {
    let app = bellatrix::Bellatrix::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
