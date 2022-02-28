use crate::utils;
use eframe::egui::{self, Color32, Hyperlink, Layout};

const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
const CYAN: Color32 = Color32::from_rgb(0, 255, 255);

pub struct BotLog {
    date: String,
    text: String,
    tx_hash: String,
}

#[derive(Default)]
pub struct Bellatrix {
    pub logs: Vec<BotLog>,

    pub private_key: String,

    pub address: String,

    // this how you opt-out of serialization of a member
    #[cfg_attr(feature = "persistence", serde(skip))]
    pub force_buy_percent: f32,

    #[cfg_attr(feature = "persistence", serde(skip))]
    pub force_sell_percent: f32,

    #[cfg_attr(feature = "persistence", serde(skip))]
    pub auto_swap: bool,
}

impl Bellatrix {
    const INTERNAL_SPACE: f32 = 5.0;

    pub fn new() -> Bellatrix {
        let iter = (0..20).map(|a| BotLog {
            date: format!("date{}", a),
            text: format!("{}", a),
            tx_hash: format!("{}", a),
        });
        Bellatrix {
            logs: Vec::from_iter(iter),
            address: String::new(),
            private_key: String::new(),
            force_buy_percent: 0.0,
            force_sell_percent: 0.0,
            auto_swap: false,
        }
    }

    /// render the wallet section
    pub fn render_wallet(&mut self, ui: &mut eframe::egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Address:");
            // TODO(elsuizo:2022-02-25): validate the input
            let address_input = ui.text_edit_singleline(&mut self.address);
        });
        ui.add_space(Self::INTERNAL_SPACE);
        ui.horizontal(|ui| {
            ui.label("PrivateKey: ");
            // TODO(elsuizo:2022-02-25): validate the password
            let password_input = utils::password_ui(ui, &mut self.private_key);
        });
        ui.add_space(Self::INTERNAL_SPACE);
        ui.separator();
    }

    pub fn render_addres(&mut self, ui: &mut eframe::egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("From(Address):");
            // TODO(elsuizo:2022-02-25): validate the address
            let address_input = ui.text_edit_singleline(&mut self.address);
            if ui.button("Accept").clicked() {
                println!("dsfsdf");
                // println!("{:?}", chrono::offset::Local::now());
            }
        });
        ui.add_space(Self::INTERNAL_SPACE);
        ui.horizontal(|ui| {
            ui.label("To(Address):");
            // TODO(elsuizo:2022-02-25): validate the address
            let address_input = ui.text_edit_singleline(&mut self.address);
            // TODO(elsuizo:2022-02-26): maybe here we need change all the button color
            ui.visuals_mut().override_text_color = Some(egui::Color32::RED);
            if ui.button("Scam").clicked() {
                println!("dsfsdf");
                // println!("{:?}", chrono::offset::Local::now());
            }
        });
        ui.add_space(Self::INTERNAL_SPACE);
        ui.separator();
    }

    pub fn render_middle_section(&mut self, ui: &mut eframe::egui::Ui) {
        egui::Grid::new("outer_grid")
            .num_columns(3)
            // space between coloumns/rows
            // .spacing([30.0, 10.0])
            // this put a shadow in each row form
            // .striped(true)
            // NOTE(elsuizo:2022-02-27): the name is important to mantain the Layout
            .show(ui, |ui| {
                egui::Grid::new("internal_grid")
                    .num_columns(2)
                    .show(ui, |ui| {
                        ui.add(
                            egui::TextEdit::singleline(&mut self.address)
                                .hint_text("Write something here1"),
                        );
                        ui.add(
                            egui::TextEdit::singleline(&mut self.address)
                                .hint_text("Write something here2"),
                        );
                        ui.end_row();
                        ui.add(
                            egui::TextEdit::singleline(&mut self.address)
                                .hint_text("Write something here3"),
                        );
                        ui.add(
                            egui::TextEdit::singleline(&mut self.address)
                                .hint_text("Write something here4"),
                        );
                    });
                // NOTE(elsuizo:2022-02-27): the name is important to mantain the Layout
                egui::Grid::new("internal_grid")
                    .num_columns(1)
                    .show(ui, |ui| {
                        ui.add(
                            egui::TextEdit::singleline(&mut self.address)
                                .hint_text("Write something here5"),
                        );
                        ui.end_row();
                        ui.add(
                            egui::TextEdit::singleline(&mut self.address)
                                .hint_text("Write something here6"),
                        );
                        ui.end_row();
                        ui.add(
                            egui::TextEdit::singleline(&mut self.address)
                                .hint_text("Write something here7"),
                        );
                        ui.end_row();
                        ui.add(
                            egui::TextEdit::singleline(&mut self.address)
                                .hint_text("Write something here7"),
                        );
                    });
            });
    }

    pub fn render_new_log(&self, ui: &mut eframe::egui::Ui) {
        for element in &self.logs {
            ui.horizontal(|ui| {
                let title = format!("{}: {}", element.date, "Buy 12323 TKM - 0.23 BNB");
                ui.colored_label(WHITE, title);

                ui.style_mut().visuals.hyperlink_color = CYAN;
                // ui.add_space(PADDING);
                ui.with_layout(Layout::right_to_left(), |ui| {
                    ui.add(Hyperlink::from_label_and_url(
                        &element.text,
                        "See Tx On Explorer ⤴",
                    ));
                });
            });
        }
    }
}
