use ethers::types::Address;
use serde::{Serialize, Deserialize};

/// Parse a string into an Address
pub fn parse_address(addr: &str) -> Address {
    let addr = addr.strip_prefix("0x").unwrap_or(addr);
    addr.parse().unwrap()
}

/// Addresses on the Live Net
pub mod live {
    pub const LIVE_OPTIMISM_PROVIDER_URL_HTTPS: &str = "https://mainnet.optimism.io";
    pub const LIVE_OPTIMISM_PROVIDER_URL_WSS: &str = "wss://ws-mainnet.optimism.io";
    pub const LIVE_ETHERSCAN_ADDRESS: &str = "https://optimistic.etherscan.io";

    pub const MARKET_ADDRESS: &str = "0x7a512d3609211e719737E82c7bb7271eC05Da70d";
    pub const MARKET_ADDRESS_STRING_LITERAL: &str = "0x7a512d3609211e719737E82c7bb7271eC05Da70d";
    pub const ROUTER_ADDRESS: &str = "0x45969104EF4561cEe269B334d8CB7a99206a09e5";
    pub const BATH_HOUSE_ADDRESS: &str = "0x2F536756636ABf29e860717a10867860802AECe7";

    pub const BATH_ETH_ADDRESS: &str = "0xB0bE5d911E3BD4Ee2A8706cF1fAc8d767A550497";
    pub const BATH_WBTC_ADDRESS: &str = "0x7571CC9895D8E997853B1e0A1521eBd8481aa186";
    pub const BATH_USDC_ADDRESS: &str = "0xe0e112e8f33d3f437D1F895cbb1A456836125952";
    pub const BATH_DAI_ADDRESS: &str = "0x60daEC2Fc9d2e0de0577A5C708BcaDBA1458A833";
    pub const BATH_USDT_ADDRESS: &str = "0xfFBD695bf246c514110f5DAe3Fa88B8c2f42c411";
}

/// Addresses on the Kovan testnet
pub mod kovan {
    pub const KOVAN_OPTIMISM_PROVIDER_URL_HTTPS: &str = "https://kovan.optimism.io";
    pub const KOVAN_OPTIMISM_PROVIDER_URL_WSS: &str = "wss://ws-kovan.optimism.io";
    pub const KOVAN_ETHERSCAN_ADDRESS: &str = "https://kovan-optimistic.etherscan.io";

    pub const MARKET_ADDRESS: &str = "0x5ddDa7DF721272106af1904abcc64E76AB2019d2";
    pub const ROUTER_ADDRESS: &str = "0xb90Bf3Eeb555100D0A6A6a34eEF4e95619F9e437";
    pub const BATH_HOUSE_ADDRESS: &str = "0x235aCb07E3CD3b7eC443257De331198c0301F44A";

    pub const BATH_ETH_ADDRESS: &str = "0x5790AedddfB25663f7dd58261De8E96274A82BAd";
    pub const BATH_WBTC_ADDRESS: &str = "0xE50C690A90fCF570fd1314d9310522BC11505A83";
    pub const BATH_USDC_ADDRESS: &str = "0x5BE0ba4d0dF55Dd5CE6A805983Ca79622926b5B6";
    pub const BATH_DAI_ADDRESS: &str = "0xe13eFeA14c0941c230C824EDfde5A03fb1F0a827";
    pub const BATH_USDT_ADDRESS: &str = "0xD9619882d9dB19AC83A237E3F8B411e3cf1E0716";
    pub const BATH_RPL_ADDRESS: &str = "0x2704D2847d5E278B58DA898c3f3D406d704A55d6";
    pub const BATH_SNX_ADDRESS: &str = "0x02A0ff3179a7E1f09D93c6C1df22520248128dbd";
    pub const BATH_UNI_ADDRESS: &str = "0xddB9f3DCD2465e7E755ED33455DD562DD555F875";
    pub const BATH_LINK_ADDRESS: &str = "0x5aa29Af7b7a403E00a3F02FA54218B56134b187f";
    pub const BATH_OHM_ADDRESS: &str = "0x3495b1248253eC9289C560C5d3EB3c7191637236";

    pub const USDC_ERC20_ADDRESS: &str = "0x4bec326fe1BeF34c4858A1De3906c7F52a95A682";
    pub const DAI_ERC20_ADDRESS: &str = "0x83DB01411E8c5b0bcaa0850E7fd90bDF7E180205";
    pub const USDT_ERC20_ADDRESS: &str = "0x59C4c66275E0Af76A81ac149b3aBc0d4BEE10672";
    pub const ETH_ERC20_ADDRESS: &str = "0x4200000000000000000000000000000000000006";
    pub const WBTC_ERC20_ADDRESS: &str = "0xd7183DBD7562D0CFe6cA7685eC67eA4c055dE0D7";
    pub const RPL_ERC20_ADDRESS: &str = "0xf897B92261BD23E28124556DaDEa3F2281d6bf8a";
    pub const SNX_ERC20_ADDRESS: &str = "0xB3614012E72CF88fA606650A701370e42f58f567";
    pub const UNI_ERC20_ADDRESS: &str = "0xf2592d37c383Ad3Ecf998C0D17Cfc9A12384b401";
    pub const LINK_ERC20_ADDRESS: &str = "0xBFcE899d931E1e193D236ab525D24AFEB9e0ae62";
    pub const OHM_ERC20_ADDRESS: &str = "0x4400232C65bab8265eb1025dDd85BF88573E8b54";
}
