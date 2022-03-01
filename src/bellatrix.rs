use crate::utils;
use crate::CYAN;
use crate::WHITE;
use eframe::egui::{self, Color32, Hyperlink, Layout};

#[derive(PartialEq, Debug)]
pub enum TokenPool {
    BNB,
    BUSD,
    USDT,
    USDC,
}

impl Default for TokenPool {
    fn default() -> Self {
        Self::BNB
    }
}

#[derive(PartialEq, Debug)]
pub enum BNBElection {
    Spend,
    Buy,
}

impl Default for BNBElection {
    fn default() -> Self {
        Self::Spend
    }
}

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

    pub bnb_election: BNBElection,

    pub user_money: f32,

    pub token_pool: TokenPool,

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
    const WINDOWS_SIGNAL_COLOR: Color32 = Color32::from_rgb(255, 140, 0);

    pub fn new() -> Bellatrix {
        let iter = (0..20).map(|a| BotLog {
            date: format!("date{}", a),
            text: format!("{}", a),
            tx_hash: format!("{}", a),
        });
        Bellatrix {
            logs: Vec::from_iter(iter),
            address: String::new(),
            bnb_election: BNBElection::Spend,
            user_money: 0.0,
            token_pool: TokenPool::BNB,
            private_key: String::new(),
            force_buy_percent: 0.0,
            force_sell_percent: 0.0,
            auto_swap: false,
        }
    }

    /// render the wallet section
    pub fn render_wallet_section(&mut self, ui: &mut eframe::egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Address:");
            // TODO(elsuizo:2022-02-25): validate the input
            let address_input = ui
                .text_edit_singleline(&mut self.address)
                .on_hover_text("write the address here");
        });
        ui.add_space(Self::INTERNAL_SPACE);
        ui.horizontal(|ui| {
            ui.label("PrivateKey: ");
            // TODO(elsuizo:2022-02-25): validate the password
            let password_input = utils::password_ui(ui, &mut self.private_key)
                .on_hover_text("write the private key here");
        });
        ui.add_space(Self::INTERNAL_SPACE);
        ui.separator();
    }

    pub fn render_addres_section(&mut self, ui: &mut eframe::egui::Ui) {
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
            let address_input = ui
                .text_edit_singleline(&mut self.address)
                .on_hover_text("write the address here");
            // TODO(elsuizo:2022-02-26): if orange color(Windows) / Scam(Macbook) is a signal check
            // before buying
            ui.visuals_mut().override_text_color = Some(Self::WINDOWS_SIGNAL_COLOR);
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
            .num_columns(2)
            // space between coloumns/rows
            // .spacing([30.0, 10.0])
            // this put a shadow in each row form
            .striped(true)
            // NOTE(elsuizo:2022-02-27): the name is important to mantain the Layout
            .show(ui, |ui| {
                egui::Grid::new("internal_grid")
                    .num_columns(2)
                    .spacing([2.0, Self::INTERNAL_SPACE])
                    .show(ui, |ui| {
                        if ui.button("BNB Balance").clicked() {
                            println!("dsfsdf");
                            // println!("{:?}", chrono::offset::Local::now());
                        }
                        ui.label("9.68");
                        ui.end_row();
                        egui::ComboBox::from_label("Select one!")
                            .selected_text(format!("{:?}", self.bnb_election))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut self.bnb_election,
                                    BNBElection::Spend,
                                    "Spend",
                                );
                                ui.selectable_value(
                                    &mut self.bnb_election,
                                    BNBElection::Buy,
                                    "Buy",
                                );
                            });
                        // TODO(elsuizo:2022-02-28): maybe here its not a good idea have a
                        // slider...
                        ui.add(egui::Slider::new(&mut self.user_money, 0f32..=1000.0).suffix("$"));
                        ui.end_row();
                        ui.label("Set gas limit");
                        ui.add(
                            egui::TextEdit::singleline(&mut self.address)
                                .hint_text("The gas you want to set"),
                        );
                        ui.end_row();
                        ui.label("Set gas price");
                        ui.add(
                            egui::TextEdit::singleline(&mut self.address)
                                .hint_text("The GWEI you want to set"),
                        );
                        ui.end_row();
                        ui.label("Set token Pair");
                        egui::ComboBox::from_label("Select the Pool!")
                            .selected_text(format!("{:?}", self.token_pool))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.token_pool, TokenPool::BNB, "BNB");
                                ui.selectable_value(&mut self.token_pool, TokenPool::BUSD, "BUSD");
                                ui.selectable_value(&mut self.token_pool, TokenPool::USDT, "USDT");
                                ui.selectable_value(&mut self.token_pool, TokenPool::USDC, "USDC");
                            });
                        ui.end_row();
                    });
                // NOTE(elsuizo:2022-02-27): the name is important to mantain the Layout
                egui::Grid::new("internal_grid")
                    .num_columns(1)
                    .spacing([2.0, Self::INTERNAL_SPACE + 3.0])
                    .show(ui, |ui| {
                        ui.heading("Swap configuration");
                        ui.end_row();
                        ui.checkbox(&mut self.auto_swap, "Enable Auto Swap");
                        ui.end_row();
                        ui.horizontal(|ui| {
                            ui.label("BUY");
                            ui.add(
                                egui::Slider::new(&mut self.force_buy_percent, 0.0..=100.0)
                                    .suffix("%"),
                            );
                            if ui.button("Force Buy").clicked() {
                                self.force_buy_percent += 1.0;
                            }
                        });
                        ui.end_row();
                        ui.horizontal(|ui| {
                            ui.label("Sell");
                            ui.add(
                                egui::Slider::new(&mut self.force_sell_percent, 0.0..=100.0)
                                    .suffix("%"),
                            );
                            if ui.button("Force Sell").clicked() {
                                self.force_sell_percent += 1.0;
                            }
                        });
                    });
            });

        ui.add_space(Self::INTERNAL_SPACE);
        ui.separator();
    }

    pub fn render_token_wallet_section(&mut self, ui: &mut eframe::egui::Ui) {}

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
