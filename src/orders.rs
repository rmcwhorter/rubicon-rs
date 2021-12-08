use serde::{Serialize, Deserialize};
use ethers::types::{Address, U256};

use crate::currency::Currency;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum OrderType {
    MarketBuy,
    MarketSell,
    LimitBuy(U256),
    LimitSell(U256),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketOrder {
    is_buy: bool,
    source_currency: Currency,
    target_currency: Currency,
    source_quantity: U256,
    user: Address,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LimitOrder {
    is_buy: bool,
    source_currency: Currency,
    target_currency: Currency,
    source_quantity: U256,
    boundary_price: U256,
    user: Address,
}