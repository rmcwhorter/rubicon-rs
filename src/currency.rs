use crate::addresses::{kovan, parse_address};
use serde::{Serialize, Deserialize};
use ethers::types::Address;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Copy)]
pub enum Currency {
    WETH,
    USDC,
    USDT,
    WBTC,
    DAI,
    RPL,
    SNX,
    UNI,
    LINK,
    OHM,
    IERC20(Address),
}

impl std::string::ToString for Currency {
    fn to_string(&self) -> String {
        match self {
            Currency::WETH => "WETH".to_owned(),
            Currency::USDC => "USDC".to_owned(),
            Currency::USDT => "USDT".to_owned(),
            Currency::WBTC => "WBTC".to_owned(),
            Currency::DAI => "DAI".to_owned(),
            Currency::RPL => "RPL".to_owned(),
            Currency::SNX => "SNX".to_owned(),
            Currency::UNI => "UNI".to_owned(),
            Currency::LINK => "LINK".to_owned(),
            Currency::OHM => "OHM".to_owned(),
            Currency::IERC20(addr) => format!("IERC20({})", addr),
        }
    }
}

impl Currency {
    pub fn kovan_address(&self) -> Address {
        match &self {
            Currency::WETH => parse_address(kovan::ETH_ERC20_ADDRESS),
            Currency::USDC => parse_address(kovan::USDC_ERC20_ADDRESS),
            Currency::USDT => parse_address(kovan::USDT_ERC20_ADDRESS),
            Currency::WBTC => parse_address(kovan::WBTC_ERC20_ADDRESS),
            Currency::DAI => parse_address(kovan::DAI_ERC20_ADDRESS),
            Currency::RPL => parse_address(kovan::RPL_ERC20_ADDRESS),
            Currency::SNX => parse_address(kovan::SNX_ERC20_ADDRESS),
            Currency::UNI => parse_address(kovan::UNI_ERC20_ADDRESS),
            Currency::LINK => parse_address(kovan::LINK_ERC20_ADDRESS),
            Currency::OHM => parse_address(kovan::OHM_ERC20_ADDRESS),
            Currency::IERC20(x) => *x,
        }   
    }
}