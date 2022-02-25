mod bellatrix;
mod utils;

use eframe::{epi, epi::App, run_native, NativeOptions};

use std::{borrow::Cow, iter::FromIterator};

use crate::bellatrix::Bellatrix;
use eframe::egui::{
    self, Button, CentralPanel, Color32, Context, FontDefinitions, FontFamily, Hyperlink, Label,
    Layout, RichText, ScrollArea, Separator, TextStyle, TopBottomPanel, Ui, Vec2, Visuals,
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
            .text_style(TextStyle::Monospace),
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
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        ctx.set_visuals(Visuals::dark());

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

            ui.horizontal(|ui| {
                ui.label("API KEY: ");
                ui.text_edit_singleline(&mut self.private_key);

                if ui.button("Set API KEY").clicked() {
                    println!("dsfsdf");
                    println!("{:?}", chrono::offset::Local::now());
                }
            });

            ui.label("");
            ui.horizontal(|ui| {
                ui.label("PrivateKey: ");
                // NOTE(elsuizo:2022-02-25): this should be not visible and only toggle when the
                // user want
                ui.add(utils::password(&mut self.private_key));

                if ui.button("Load account").clicked() {
                    println!("dsfsdf");
                    println!("{:?}", chrono::offset::Local::now());
                }
            });

            ui.horizontal(|ui| {
                ui.label("Contract: ");
                ui.text_edit_singleline(&mut self.private_key);

                if ui.button("Set Contract").clicked() {
                    println!("dsfsdf");
                    println!("{:?}", chrono::offset::Local::now());
                }
            });

            ui.label("");
            ui.horizontal(|ui| {
                ui.label("Swap From: ");
                ui.text_edit_singleline(&mut self.private_key);

                if ui.button("Set Contract").clicked() {
                    println!("dsfsdf");
                    println!("{:?}", chrono::offset::Local::now());
                }
            });
            ui.horizontal(|ui| {
                ui.label("Swap To: ");
                ui.text_edit_singleline(&mut self.private_key);

                if ui.button("Set Contract").clicked() {
                    println!("dsfsdf");
                    println!("{:?}", chrono::offset::Local::now());
                }
            });

            // TODO(elsuizo:2022-02-25): remove all the empty labels like this
            // These are equivalent:
            ui.label("");
            ui.heading("Swap configuration");

            // BUY
            ui.label("");
            ui.horizontal(|ui| {
                ui.checkbox(&mut self.auto_swap, "Enable Auto Swap");
            });

            ui.horizontal(|ui| {
                ui.label("BUY");
                ui.add(egui::Slider::new(&mut self.force_buy_percent, 0.0..=100.0));
                ui.label("%");
                if ui.button("Force Buy").clicked() {
                    self.force_buy_percent += 1.0;
                }

                // TODO(elsuizo:2022-02-25): remove all the "\t" like this
                ui.label("\t\t");

                ui.horizontal(|ui| {
                    ui.label("SELL");
                    ui.label("");
                });

                // TODO(elsuizo:2022-02-25): maybe this is not necessary to be float
                ui.add(egui::Slider::new(&mut self.force_sell_percent, 0.0..=100.0));
                ui.label("%");
                if ui.button("Force Sell").clicked() {
                    self.force_sell_percent += 1.0;
                }
            });

            ui.label("");

            ui.heading("Log: ");
            ui.vertical(|ui| {
                ui.label("2022/02/06 02:39:16:  dsfsdf sdfs df sdfsdfsd dsf  sdf");
                ui.label("2022/02/06 02:39:16:  dsfsdf sdfs df sdfsdfs  sdf");
                ui.label("2022/02/06 02:39:16:  dsfsdf sdfs df sdfsdfsd");
                ui.label("2022/02/06 02:39:16:  dsfsdf sdfs df sdfsdfsd dsdsfsdff  sdf: ");
                ui.label("2022/02/06 02:39:16:  dsfsdf sdfs df sdfsdfsd dssdf sdf: ");
            });

            //ui.label(l);

            egui::warn_if_debug_build(ui);
        });
    }

    /// Called once before the first frame.
    fn setup(
        &mut self,
        _ctx: &egui::Context,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }
    }

    /// Called by the frame work to save state before shutdown.
    /// Note that you must enable the `persistence` feature for this to work.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    fn name(&self) -> &str {
        "eframe template"
    }
}

fn main() {
    let app = bellatrix::Bellatrix::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
