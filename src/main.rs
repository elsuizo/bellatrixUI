mod bellatrix;
mod utils;
use eframe::epi;

use crate::bellatrix::Bellatrix;
use eframe::egui::{self, Context, Separator, TopBottomPanel, Ui, Vec2, Visuals};
use std::env;

pub const PADDING: f32 = 5.0;

impl epi::App for Bellatrix {
    fn update(&mut self, ctx: &Context, frame: &epi::Frame) {
        // NOTE(elsuizo:2022-03-18): teniendo todos las partes principales de la UI separadas cada
        // una con un metodo hace que sea mas facil cuando queremos "apagar" alguna de las partes
        // simplemente comentando esa linea de codigo
        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_top_panel(ui, ctx, frame);

            self.render_header(ui);

            self.render_footer(ui, ctx);

            self.render_wallet_section(ui);

            self.render_addres_section(ui);

            self.render_middle_section(ui);

            // TODO(elsuizo:2022-03-18): esta seccion la dejamos para mas adelante
            // self.render_token_wallet_section(ui);

            self.render_take_profit_section(ui);

            self.render_tracking_information_section(ui);

            self.render_activate_stop_section(ui);

            // TODO(elsuizo:2022-03-18): esto es el intento para cambiar las fuentes de tamanio
            // self.font_id_ui(ui);
            egui::warn_if_debug_build(ui);
        });
    }

    fn setup(&mut self, ctx: &Context, _frame: &epi::Frame, _storage: Option<&dyn epi::Storage>) {
        // NOTE(elsuizo:2022-03-06): una manera de agrandar la fuente parece que puede ser esta...
        // ctx.set_pixels_per_point(1.7);
        // enable dark mode
        ctx.request_repaint();
        ctx.set_visuals(Visuals::dark());
        // self.configure_fonts(ctx);

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
    dotenv::dotenv().ok();
    let mut app = bellatrix::Bellatrix::new();
    app.load(
        &env::var("ACCOUNT_ADDRESS").unwrap(),
        &env::var("PRIVATE_TEST_KEY").unwrap(),
    );
    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(Vec2::new(500.0, 500.0));
    eframe::run_native(Box::new(app), native_options);
}
