use serde::{Deserialize, Serialize};
use web3::types::U256;

/// User code
// TODO(elsuizo:2022-03-18): hacer los getters y setters para este type
#[derive(Debug, Default)]
pub struct User {
    /// user configuration for the app
    pub config: UserConfig,
    /// balance of the user account
    pub balance: U256,
    /// cryptocurrency address
    pub crypto_address: String,
    /// contract to buy address
    pub contract_address: String,

    pub force_buy_percent: f32,

    pub force_sell_percent: f32,

    pub auto_swap: bool,
}

impl User {
    pub fn new() -> Self {
        // load the config from .config/bellatrix/bellatrix.toml
        let config: UserConfig = confy::load("bellatrix").unwrap_or_default();
        Self {
            config,
            ..Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct UserConfig {
    wallet_address: String,
    private_key: String,
    take_profit: f32,
    stop_loss: f32,
    gas_limit: f32,
    slippage: f32,
}

impl UserConfig {
    pub fn new(
        wallet_address: String,
        private_key: String,
        take_profit: f32,
        stop_loss: f32,
        gas_limit: f32,
        slippage: f32,
    ) -> Self {
        Self {
            wallet_address,
            private_key,
            take_profit,
            stop_loss,
            gas_limit,
            slippage,
        }
    }
    //-------------------------------------------------------------------------
    //                        getters
    //-------------------------------------------------------------------------
    pub fn get_wallet_address(&self) -> &str {
        &self.wallet_address
    }

    pub fn get_private_key(&self) -> &str {
        &self.private_key
    }

    pub fn get_take_profit(&self) -> f32 {
        self.take_profit
    }

    pub fn get_stop_loss(&self) -> f32 {
        self.stop_loss
    }

    pub fn get_gas_limit(&self) -> f32 {
        self.gas_limit
    }

    pub fn get_slippage(&self) -> f32 {
        self.slippage
    }

    //-------------------------------------------------------------------------
    //                        setters
    //-------------------------------------------------------------------------
    pub fn set_take_profit(&mut self, amount: f32) {
        self.take_profit = amount;
    }

    pub fn set_stop_loss(&mut self, amount: f32) {
        self.stop_loss = amount;
    }

    pub fn set_slippage(&mut self, amount: f32) {
        self.slippage = amount;
    }
}
