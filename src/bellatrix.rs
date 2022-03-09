use crate::egui::{
    self, Color32, Context, FontData, FontDefinitions, FontFamily, Hyperlink, Layout, Slider,
    TextStyle,
};
use crate::utils;
use crate::CYAN;
use crate::WHITE;
use std::borrow::Cow;

#[derive(PartialEq, Debug, Clone)]
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

#[derive(Clone, PartialEq, Debug)]
pub enum BNBElection {
    Spend,
    Buy,
}

impl Default for BNBElection {
    fn default() -> Self {
        Self::Spend
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TrackingInformation {
    BscCode,
    Holders,
    Pancake,
    Poocoin,
    Dextools,
    Honeypot,
}

impl Default for TrackingInformation {
    fn default() -> Self {
        Self::BscCode
    }
}

#[derive(Clone, PartialEq)]
pub struct BotLog {
    date: String,
    text: String,
    tx_hash: String,
}

#[derive(Default)]
pub struct Bellatrix {
    pub font_id: egui::FontId,

    pub logs: Vec<BotLog>,

    pub private_key: String,

    pub address: String,

    pub bnb_election: BNBElection,

    pub user_money: f32,

    pub token_pool: TokenPool,

    pub token_amount_sell: String,

    pub token_amount_sell_percent: f32,

    pub tracking_information: TrackingInformation,

    // this how you opt-out of serialization of a member
    #[cfg_attr(feature = "persistence", serde(skip))]
    pub force_buy_percent: f32,

    #[cfg_attr(feature = "persistence", serde(skip))]
    pub force_sell_percent: f32,

    #[cfg_attr(feature = "persistence", serde(skip))]
    pub auto_swap: bool,
}

impl Bellatrix {
    //-------------------------------------------------------------------------
    //                        type constants
    //-------------------------------------------------------------------------
    const INTERNAL_SPACE: f32 = 5.0;
    const ROWS_SPACE: f32 = 10.0;
    const COLUMNS_SPACE: f32 = 10.0;
    const WINDOWS_SIGNAL_COLOR: Color32 = Color32::from_rgb(255, 140, 0);
    const ERROR_ADDRESS_COLOR: Color32 = Color32::from_rgb(255, 0, 0);
    const GOOD_ADDRESS_COLOR: Color32 = Color32::from_rgb(0, 255, 0);
    const VALID_ADDRESS_LENGTH: usize = 42;

    pub fn configure_fonts(&mut self, ctx: &Context) {
        let mut font_definitions = FontDefinitions::default();
        font_definitions.font_data.insert(
            "MesloLGS".to_string(),
            FontData::from_static(include_bytes!("../MesloLGS_NF_Regular.ttf")),
        );
        // TODO(elsuizo:2022-03-04): esto no anda en esta version de eframe
        // font_definitions
        //     .families
        //     .insert(TextStyle::Heading, (FontFamily::Proportional, 35.0));
        // font_definitions
        //     .families
        //     .insert(TextStyle::Body, (FontFamily::Proportional, 20.0));
        font_definitions
            .families
            .get_mut(&FontFamily::Monospace)
            .unwrap()
            .insert(0, "MesloLGS".to_string());

        ctx.set_fonts(font_definitions);
    }

    pub fn new() -> Bellatrix {
        let iter = (0..20).map(|a| BotLog {
            date: format!("date{}", a),
            text: format!("{}", a),
            tx_hash: format!("{}", a),
        });
        Bellatrix {
            font_id: Default::default(),
            logs: Vec::from_iter(iter),
            address: String::new(),
            bnb_election: BNBElection::Spend,
            user_money: 0.0,
            token_pool: TokenPool::BNB,
            private_key: String::new(),
            token_amount_sell: String::new(),
            token_amount_sell_percent: 0.0,
            tracking_information: TrackingInformation::default(),
            force_buy_percent: 0.0,
            force_sell_percent: 0.0,
            auto_swap: false,
        }
    }

    // TODO(elsuizo:2022-03-06): esto no anda
    pub fn font_id_ui(&mut self, ui: &mut eframe::egui::Ui) {
        let families = ui.fonts().families();
        ui.horizontal(|ui| {
            ui.add(Slider::new(&mut self.font_id.size, 4.0..=40.0).max_decimals(0));
            // for alternative in families {
            //     let text = alternative.to_string();
            //     ui.radio_value(&mut self.font_id.family, alternative, text);
            // }
        });
    }

    /// render the wallet section
    pub fn render_wallet_section(&mut self, ui: &mut eframe::egui::Ui) {
        ui.add_space(Self::INTERNAL_SPACE);

        // NOTE(elsuizo:2022-03-08): con frame == false lo que hace es no renderizar al boton en si
        // sino que hace que parezca un Label
        // validation feedback for the user
        let good_address = egui::Button::new(
            egui::RichText::new(" ✔ ".to_string()).color(Self::GOOD_ADDRESS_COLOR),
        )
        .frame(false);
        let bad_address = egui::Button::new(
            egui::RichText::new(" 🗙 ".to_string()).color(Self::ERROR_ADDRESS_COLOR),
        )
        .frame(false);

        // TODO(elsuizo:2022-03-08): hay que ponerle mas size a estos dos campos(porque no llega a
        // renderizar una address completa osea deja caracteres afuera)
        ui.horizontal(|ui| {
            ui.label("Address:");
            // TODO(elsuizo:2022-02-25): validate the input
            let address_input = ui
                .text_edit_singleline(&mut self.address)
                .on_hover_text("write a valid address here");

            // render the validation feedback message
            if utils::validate_address_length(&self.address, Self::VALID_ADDRESS_LENGTH) {
                ui.add(good_address);
            } else {
                ui.add(bad_address);
            }
        });

        ui.add_space(Self::INTERNAL_SPACE);

        let good_private_key = egui::Button::new(
            egui::RichText::new(" ✔ ".to_string()).color(Self::GOOD_ADDRESS_COLOR),
        )
        .frame(false);
        let bad_private_key = egui::Button::new(
            egui::RichText::new(" 🗙 ".to_string()).color(Self::ERROR_ADDRESS_COLOR),
        )
        .frame(false);

        ui.horizontal(|ui| {
            ui.label("PrivateKey: ");
            // TODO(elsuizo:2022-02-25): validate the password
            let password_input = utils::password_ui(ui, &mut self.private_key)
                .on_hover_text("write the private key here");
            // render the validation feedback message
            if utils::validate_address_length(&self.address, Self::VALID_ADDRESS_LENGTH) {
                ui.add(good_private_key);
            } else {
                ui.add(bad_private_key);
            }
        });

        ui.add_space(Self::INTERNAL_SPACE);
        ui.separator();
    }

    pub fn render_addres_section(&mut self, ui: &mut eframe::egui::Ui) {
        ui.add_space(Self::INTERNAL_SPACE);

        ui.horizontal(|ui| {
            ui.label("From(Address):");
            // TODO(elsuizo:2022-02-25): validate the address
            let address_input = ui.text_edit_singleline(&mut self.address);
            if ui.button("Accept").clicked() {
                println!("address input");
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
                println!("check the signal before buying");
            }
        });

        ui.add_space(Self::INTERNAL_SPACE);
        ui.separator();
    }

    pub fn render_middle_section(&mut self, ui: &mut eframe::egui::Ui) {
        ui.add_space(Self::INTERNAL_SPACE);

        egui::Grid::new("outer_grid")
            .num_columns(2)
            // space between coloumns/rows
            .spacing([Self::ROWS_SPACE, Self::COLUMNS_SPACE])
            // this put a shadow in each row form
            .striped(true)
            // NOTE(elsuizo:2022-02-27): the name is important to mantain the Layout
            .show(ui, |ui| {
                egui::Grid::new("internal_grid")
                    .num_columns(2)
                    .spacing([Self::ROWS_SPACE, Self::COLUMNS_SPACE])
                    .show(ui, |ui| {
                        if ui.button("BNB Balance").clicked() {
                            println!("check balance");
                        }
                        ui.label("1.233");
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
                        ui.add(
                            egui::Slider::new(&mut self.user_money, 0f32..=1000.0)
                                .text("put text here")
                                .suffix("$"),
                        );
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
                egui::Grid::new("internal_grid1")
                    .num_columns(1)
                    .spacing([Self::ROWS_SPACE, Self::COLUMNS_SPACE])
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

    pub fn render_token_wallet_section(&mut self, ui: &mut eframe::egui::Ui) {
        ui.add_space(Self::INTERNAL_SPACE);

        ui.horizontal(|ui| {
            if ui.button("Tk Balance").clicked() {
                println!("Token balance calculation ...");
            }
            ui.label("0.3773");
            ui.add(egui::Slider::new(&mut self.token_amount_sell_percent, 0.0..=100.0).suffix("%"));
            // TODO(elsuizo:2022-03-01): handle this response
            let exact_amout_to_sell = ui
                .text_edit_singleline(&mut self.token_amount_sell)
                .on_hover_text_at_pointer("write the exact amount to sell here");
            if ui.button("Sell").clicked() {
                println!("Sell!!!");
            }
        });

        ui.add_space(Self::INTERNAL_SPACE);
        ui.separator();
    }

    pub fn render_tracking_information_section(&mut self, ui: &mut eframe::egui::Ui) {
        ui.add_space(Self::INTERNAL_SPACE);

        ui.horizontal(|ui| {
            ui.label("Tracking information: ");

            ui.selectable_value(
                &mut self.tracking_information,
                TrackingInformation::BscCode,
                "BscCode",
            );
            ui.selectable_value(
                &mut self.tracking_information,
                TrackingInformation::Holders,
                "Holders",
            );
            ui.selectable_value(
                &mut self.tracking_information,
                TrackingInformation::Pancake,
                "Pancake",
            );
            ui.selectable_value(
                &mut self.tracking_information,
                TrackingInformation::Poocoin,
                "Poocoin",
            );
            ui.selectable_value(
                &mut self.tracking_information,
                TrackingInformation::Dextools,
                "Dextools",
            );
            ui.selectable_value(
                &mut self.tracking_information,
                TrackingInformation::Honeypot,
                "Honeypot",
            );
        });

        ui.add_space(Self::INTERNAL_SPACE);

        ui.horizontal(|ui| {
            // TODO(elsuizo:2022-02-25): validate the address
            let address_input = ui.text_edit_singleline(&mut self.address);
            if ui.button(" ⎆ ").clicked() {
                println!("Check output transaction and warning users");
            }
        });

        ui.add_space(Self::INTERNAL_SPACE);
        ui.separator();
    }

    pub fn render_new_log(&self, ui: &mut eframe::egui::Ui) {
        ui.add_space(Self::INTERNAL_SPACE);

        for element in &self.logs {
            ui.horizontal(|ui| {
                let title = format!("{}: {}", element.date, "Buy 12323 TKM - 0.23 BNB");
                ui.colored_label(Color32::DARK_RED, title);

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
