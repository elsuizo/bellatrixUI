use crate::egui::{
    self, Color32, Context, FontData, FontDefinitions, FontFamily, Hyperlink, Label, Layout,
    RichText, ScrollArea, Slider, TextStyle, TopBottomPanel,
};
use crate::user::{User, UserConfig};
use crate::utils;
use eframe::epi;
// use std::borrow::Cow;
use std::sync::mpsc::{Receiver, SyncSender};
use web3::types::{Address, H160, U256};
use web3_rust_wrapper::Web3Manager;

use std::{
    sync::mpsc::{channel, sync_channel},
    thread,
};
/// load the user with the wallet address and the private key
pub fn load(web3: &mut Web3Wrapper, plain_address: &str, private_key: &str) {
    async_std::task::block_on(async {
        web3.web3m.load_account(&plain_address, &private_key).await;
    })
}

pub enum Msg {
    UserData(String, String),
}

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

// #[derive(Debug)]
pub struct Web3Wrapper {
    pub web3m: Web3Manager,
}

impl Default for Web3Wrapper {
    fn default() -> Self {
        async_std::task::block_on(async {
            let web3m = Web3Manager::new(Self::HTTP_URL, Self::WEB_SOCKET_URL).await;
            Self { web3m }
        })
    }
}

impl Web3Wrapper {
    const WEB_SOCKET_URL: &'static str =
        "wss://speedy-nodes-nyc.moralis.io/84a2745d907034e6d388f8d6/bsc/testnet/ws";
    const HTTP_URL: &'static str =
        "https://speedy-nodes-nyc.moralis.io/84a2745d907034e6d388f8d6/bsc/testnet";
}

//-------------------------------------------------------------------------
//                        UI code
//-------------------------------------------------------------------------

// #[derive(Default)]
pub struct Bellatrix {
    // users: Vec<User>,
    pub user: User,

    gas_limit_input: String,

    gas_price_input: String,

    pub font_id: egui::FontId,

    pub logs: Vec<BotLog>,

    pub bnb_election: BNBElection,

    pub user_money_input: f32,

    pub token_pool: TokenPool,

    pub token_amount_sell: String,

    pub token_amount_sell_percent: f32,

    pub tracking_information: TrackingInformation,

    pub web3m_wrapper: Web3Wrapper,

    pub bot_state: bool,

    pub inputs: (bool, bool, bool),

    pub app_tx: Option<SyncSender<Msg>>,
}

impl Bellatrix {
    //-------------------------------------------------------------------------
    //                        type constants
    //-------------------------------------------------------------------------
    const INTERNAL_SPACE: f32 = 5.0;
    const ROWS_SPACE: f32 = 10.0;
    const COLUMNS_SPACE: f32 = 10.0;
    const WINDOWS_SIGNAL_COLOR: Color32 = Color32::from_rgb(255, 140, 0);

    //-------------------------------------------------------------------------
    //                        getters
    //-------------------------------------------------------------------------
    pub fn get_bot_state(&self) -> bool {
        self.bot_state
    }

    pub fn check_inputs(&self) -> bool {
        self.inputs.0 && self.inputs.1
    }

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
            user: User::new(),
            gas_limit_input: Default::default(),
            gas_price_input: Default::default(),
            font_id: Default::default(),
            logs: Vec::from_iter(iter),
            bnb_election: BNBElection::Spend,
            user_money_input: 0.0,
            token_pool: TokenPool::BNB,
            token_amount_sell: String::new(),
            token_amount_sell_percent: 0.0,
            tracking_information: TrackingInformation::default(),
            web3m_wrapper: Default::default(),
            bot_state: false,
            inputs: Default::default(),
            app_tx: None,
        }
    }

    pub fn render_header(&self, ui: &mut eframe::egui::Ui) {
        ui.add_space(Self::INTERNAL_SPACE);

        ui.vertical_centered(|ui| {
            ui.add(Label::new(
                RichText::new("Account Info")
                    .color(Color32::WHITE)
                    .heading(),
            ));

            ui.vertical_centered(|ui2| {
                ui2.label(format!("{}", self.user.config.get_wallet_address()));
                ui2.label(format!("Balance: {}", utils::wei_to_eth(self.user.balance)));
            });
        });

        ui.add_space(Self::INTERNAL_SPACE);
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

    // NOTE(elsuizo: 2022-04-30): tenemos disponible el estado del bot en la variable `bot_state`
    // la idea seria hacer que cuando cambia de estado iniciar el "bot"
    // como puede ser que ande mejor que sin la espuma esta de mierda
    pub fn render_activate_stop_section(&mut self, ui: &mut eframe::egui::Ui) {
        ui.add_space(Self::INTERNAL_SPACE);

        ui.vertical_centered(|ui| {
            ui.add(Label::new(
                RichText::new("STOP / START")
                    .color(Color32::WHITE)
                    .heading(),
            ));
            let response = utils::toggle(&mut self.bot_state);
            ui.add(response);
        });
        ui.add_space(Self::INTERNAL_SPACE);
        ui.separator();
    }

    pub fn render_footer(&self, ui: &mut eframe::egui::Ui, ctx: &Context) {
        let user_input = String::new();
        let button = egui::Button::new("Button").frame(false);
        let tooltip_ui = |ui: &mut egui::Ui| {
            ui.label(egui::RichText::new(
                "Donate: 0x43aF68DcC19Bbce20F3354F31Bc1159f651643aE",
            ));
            ui.label("Click to copy");
        };

        TopBottomPanel::bottom("footer").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(10.);
                if ui.add(button).on_hover_ui(tooltip_ui).clicked() {
                    ui.output().copied_text = user_input;
                }
                utils::render_monospaced_label(ui, "Made with ♥ By Cau1dr0n 1ake");
                utils::render_monospaced_label(
                    ui,
                    "Donate: 0x43aF68DcC19Bbce20F3354F31Bc1159f651643aE",
                );
                utils::render_hyperlink(ui, "https://domain.com", "Buy API KEY");
                ui.add_space(10.);
            })
        });
    }

    pub fn render_top_panel(
        &self,
        ui: &mut eframe::egui::Ui,
        ctx: &egui::Context,
        frame: &epi::Frame,
    ) {
        ui.add_space(Self::INTERNAL_SPACE);

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
                // save button
                ui.menu_button("save", |ui| {
                    if ui.button("save parameters").clicked() {
                        confy::store(
                            "bellatrix",
                            UserConfig::new(
                                self.user.config.get_wallet_address().to_string(),
                                self.user.config.get_private_key().to_string(),
                                self.user.config.get_take_profit(),
                                self.user.config.get_stop_loss(),
                                self.user.config.get_gas_limit(),
                                self.user.config.get_slippage(),
                            ),
                        )
                        .unwrap()
                    }
                });
            });
        });

        // NOTE(elsuizo:2022-03-18): le agrego un poco mas para que quede bien
        ui.add_space(Self::INTERNAL_SPACE + 20.0);
        // ui.separator();
    }

    /// render the wallet section
    // pub fn render_wallet_section(&mut self, ui: &mut eframe::egui::Ui) {
    //     ui.add_space(Self::INTERNAL_SPACE + 20.0);
    //
    //     // NOTE(elsuizo:2022-03-08): con frame == false lo que hace es no renderizar al boton en si
    //     // sino que hace que parezca un Label
    //     // validation feedback for the user
    //     let good_address = egui::Button::new(
    //         egui::RichText::new(" ✔ ".to_string()).color(Self::GOOD_ADDRESS_COLOR),
    //     )
    //     .frame(false);
    //     let bad_address = egui::Button::new(
    //         egui::RichText::new(" 🗙 ".to_string()).color(Self::ERROR_ADDRESS_COLOR),
    //     )
    //     .frame(false);
    //
    //     // TODO(elsuizo:2022-03-08): hay que ponerle mas size a estos dos campos(porque no llega a
    //     // renderizar una address completa osea deja caracteres afuera)
    //     ui.horizontal(|ui| {
    //         ui.label("Address:");
    //         // TODO(elsuizo:2022-02-25): validate the input
    //         let address_input = ui
    //             .text_edit_singleline(&mut self.user.wallet_address)
    //             .on_hover_text("write a valid address here");
    //
    //         if address_input.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
    //             if utils::validate_address_length(
    //                 &self.user.wallet_address,
    //                 Self::VALID_ADDRESS_LENGTH,
    //             ) {
    //                 self.inputs.0 = true;
    //             } else {
    //                 self.inputs.0 = false;
    //             }
    //         }
    //         // render the validation feedback message
    //         if self.inputs.0 {
    //             ui.add(good_address);
    //         } else {
    //             ui.add(bad_address);
    //             ui.add(egui::widgets::Spinner::new());
    //         }
    //     });
    //
    //     ui.add_space(Self::INTERNAL_SPACE);
    //
    //     let good_private_key = egui::Button::new(
    //         egui::RichText::new(" ✔ ".to_string()).color(Self::GOOD_ADDRESS_COLOR),
    //     )
    //     .frame(false);
    //     let bad_private_key = egui::Button::new(
    //         egui::RichText::new(" 🗙 ".to_string()).color(Self::ERROR_ADDRESS_COLOR),
    //     )
    //     .frame(false);
    //
    //     ui.horizontal(|ui| {
    //         ui.label("PrivateKey: ");
    //         // TODO(elsuizo:2022-02-25): validate the password
    //         let password_input = utils::password_ui(ui, &mut self.user.private_key)
    //             .on_hover_text("write the private key here");
    //         // render the validation feedback message
    //         if ui.input().key_pressed(egui::Key::Enter) {
    //             if utils::validate_address_length(
    //                 &self.user.private_key,
    //                 Self::VALID_PRIVATE_KEY_LENGTH,
    //             ) {
    //                 self.inputs.1 = true;
    //             } else {
    //                 self.inputs.1 = false;
    //             }
    //         }
    //         // render the validation feedback message
    //         if self.inputs.1 {
    //             ui.add(good_private_key);
    //         } else {
    //             ui.add(bad_private_key);
    //             ui.add(egui::widgets::Spinner::new());
    //         }
    //     });
    //
    //     ui.add_space(Self::INTERNAL_SPACE);
    //     ui.separator();
    // }

    pub fn render_addres_section(&mut self, ui: &mut eframe::egui::Ui) {
        ui.add_space(Self::INTERNAL_SPACE);

        ui.vertical_centered(|ui| {
            ui.add(Label::new(
                RichText::new(
                    "Select your preferred cryptocurrency and the contract you wish to purchase",
                )
                .color(Color32::WHITE)
                .heading(),
            ));
        });

        ui.add_space(Self::INTERNAL_SPACE);

        ui.horizontal(|ui| {
            ui.label("From(Address):");
            // TODO(elsuizo:2022-02-25): validate the address
            let address_input = ui.text_edit_singleline(&mut self.user.contract_address);
            if ui.button("Accept").clicked() {
                println!("address input");
            }
        });
        ui.add_space(Self::INTERNAL_SPACE);

        ui.horizontal(|ui| {
            ui.label("To Token contract:");
            // TODO(elsuizo:2022-02-25): validate the address
            let address_input = ui
                .text_edit_singleline(&mut self.user.crypto_address)
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
                        if ui.button("Balance").clicked() {
                            let accounts = &self.web3m_wrapper.web3m.accounts;
                            // only if the account are valid
                            if !accounts.is_empty() {
                                self.user.balance =
                                    self.web3m_wrapper.web3m.get_account_balance(accounts[0]);
                            }
                        }
                        // show the balance current value
                        ui.label(format!("{} wei", utils::wei_to_eth(self.user.balance)));
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
                            egui::Slider::new(&mut self.user_money_input, 0f32..=1000.0)
                                .text("put text here")
                                .suffix("＄"),
                        );
                        ui.end_row();
                        ui.label("Set gas limit");
                        ui.add(
                            egui::TextEdit::singleline(&mut self.gas_limit_input)
                                .hint_text("The gas you want to set"),
                        );
                        ui.end_row();
                        ui.label("Set gas price");
                        ui.add(
                            egui::TextEdit::singleline(&mut self.gas_price_input)
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
                        ui.add(Label::new(
                            RichText::new("Swap configuration")
                                .color(Color32::WHITE)
                                .heading(),
                        ));
                        ui.end_row();
                        ui.checkbox(&mut self.user.auto_swap, "Enable Auto Swap");
                        ui.end_row();
                        ui.horizontal(|ui| {
                            ui.label("BUY");
                            ui.add(
                                egui::Slider::new(&mut self.user.force_buy_percent, 0.0..=100.0)
                                    .suffix(" ％"),
                            );
                            if ui.button("Force Buy").clicked() {
                                self.user.force_buy_percent += 1.0;
                            }
                        });
                        ui.end_row();
                        ui.horizontal(|ui| {
                            ui.label("Sell");
                            ui.add(
                                egui::Slider::new(&mut self.user.force_sell_percent, 0.0..=100.0)
                                    .suffix(" ％"),
                            );
                            if ui.button("Force Sell").clicked() {
                                self.user.force_sell_percent += 1.0;
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
            let address_input = ui.text_edit_singleline(&mut self.user.contract_address);
            if ui.button(" ⎆ ").clicked() {
                println!("Check output transaction and warning users");
            }
        });

        ui.add_space(Self::INTERNAL_SPACE);
        ui.separator();
    }

    pub fn render_take_profit_section(&mut self, ui: &mut eframe::egui::Ui) {
        ui.add_space(Self::INTERNAL_SPACE);

        ui.horizontal(|ui| {
            // take_profit slider
            let mut take_profit = self.user.config.get_take_profit();
            ui.label("Take profit");
            ui.add(
                egui::Slider::new(&mut take_profit, 0.0..=100.0)
                    // .text("put text here")
                    .suffix(" ％"),
            );
            self.user.config.set_take_profit(take_profit);

            // stop loss slider
            ui.label("Stop loss");
            let mut stop_loss = self.user.config.get_stop_loss();
            ui.add(
                egui::Slider::new(&mut stop_loss, 0.0..=100.0)
                    // .text("put text here")
                    .suffix(" ％"),
            );
            self.user.config.set_stop_loss(stop_loss);

            // slippage slider
            ui.label("Slippage");
            let mut slippage = self.user.config.get_slippage();
            ui.add(
                egui::Slider::new(&mut slippage, 0.0..=100.0)
                    // .text("put text here")
                    .suffix(" ％"),
            );
            self.user.config.set_slippage(slippage);
        });

        ui.add_space(Self::INTERNAL_SPACE);
        ui.separator();
    }

    // pub fn render_bot_logs(&self, ui: &mut eframe::egui::Ui) {}

    // TODO(elsuizo:2022-03-20): ver bien como hay que hacer el scroll para que aparezca solo el
    // ultimo valor visible y los otros vayan pasando a un scroll
    pub fn render_new_log(&self, ui: &mut eframe::egui::Ui) {
        ui.add_space(Self::INTERNAL_SPACE);
        let row_height = 1.0;
        let total_rows = 100;

        // TODO(elsuizo:2022-03-20): faltaria calcular bien el parametro `max_height` que es el que
        // maneja el tamanio que se ve del ScrollArea
        ui.vertical_centered(|ui| {
            egui::ScrollArea::vertical().max_height(10.0).show_rows(
                ui,
                row_height,
                total_rows,
                |ui, row_range| {
                    for row in row_range {
                        let text = format!("BotLog here!!! ---> {}/{}", row + 1, total_rows);
                        ui.label(egui::RichText::new(text).size(20.0).color(Color32::WHITE));
                    }
                },
            );
        });
        ui.add_space(Self::INTERNAL_SPACE);
        ui.separator();
    }
}
