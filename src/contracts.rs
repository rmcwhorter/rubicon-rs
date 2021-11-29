use anyhow::{anyhow, Result};
use ethers::{
    contract::{abigen, Contract},
    providers::{Http, Provider},
    types::Address,
};
use ethers_core::{abi::Abi};
use serde_json::{from_str, Value};
use std::fs;
use std::path::Path;
use std::sync::Arc;

use crate::addresses;
use crate::providers;
use addresses::parse_address;
/// formats a hex address (e.g. "0x7a512d3609211e719737E82c7bb7271eC05Da70d") in the form "etherscan:0x7a512d3609211e719737E82c7bb7271eC05Da70d"



type HttpContract = Contract<Provider<Http>>;

pub fn gen_contract(address: Address, abi: Abi, client: Provider<Http>) -> Result<HttpContract> {
    Ok(Contract::new(address, abi, client))
}

fn build_contract_from_json_dir<P: AsRef<Path>>(
    dir_path: P,
    contract_name: &str,
    address: Address,
    client: Provider<Http>,
) -> Result<HttpContract> {
    let file_path = dir_path.as_ref().join(format!("{}.json", contract_name));
    build_contract_from_json_path(file_path, address, client)
}

pub fn parse_abi_from_json(json_str:  &str) -> Result<Abi> {
    let v: Value = from_str(json_str)?;
    let abi: Abi = from_str(&v["abi"].to_string())?; // EXTREMELY SKETCHY AND SLOW, PROBABLY
    return Ok(abi);
}

fn build_contract_from_json_path<P: AsRef<Path>>(
    file_path: P,
    address: Address,
    client: Provider<Http>,
) -> Result<HttpContract> {
    let contents = fs::read_to_string(file_path)?;

    let v: Value = from_str(&contents)?;
    let abi: Abi = from_str(&v["abi"].to_string())?; // EXTREMELY SKETCHY AND SLOW, PROBABLY

    gen_contract(address, abi, client)
}

fn fmt_etherscan_address(address: &str, net: Network) -> String {
    match net {
        Live => format!(
            "{}/address/{}",
            addresses::live::LIVE_ETHERSCAN_ADDRESS,
            address
        ),
        Kovan => format!(
            "{}/address/{}",
            addresses::kovan::KOVAN_ETHERSCAN_ADDRESS,
            address
        ),
    }
}

/**
 * Questions for Benjamin and Forrest
 * 1) Your company looks really cool
 * 2) How's the O-ETH Kovan transition going?
 * 3) Funding?
 * 4) What's the actual plan for validating and onboarding market makers?
 *    What do you mean, "open the strategist role to anyone"?
 * 5) Other order types? Stoploss orders?
 * 6) How do I get the order book off of the exchange?
 * 7) Inventory risk?
 * 8) Race conditions for pool assets between multiple strategists?
 * 9) 
 */

pub struct StrategistSession {
    provider: Provider<Http>,
    bath_house: HttpContract,
}

#[derive(Debug, Clone, Copy)]
pub enum Network {
    Live,
    Kovan,
}

impl StrategistSession {


    /**
     * This is erroring out when we actually send the transaction. Idk why, needs to be fixed
     */
    pub async fn bootstrap(net: Network) -> Result<Self> {
        let json_dir = "abi";
        println!("Flag 1");
        // first, we get our provider
        // second, we build the bath house contract
        // third, we use the bath house contract to build all of the bath pools contracts
        // at that point we should have a complete session

        // first step
        println!("Flag 2");
        let provider = match net {
            Live => providers::live::http_provider(),
            Kovan => providers::kovan::http_provider(),
        }?;

        let bath_house_contract = match net {
            Live => build_contract_from_json_dir(json_dir, "BathHouse", parse_address(addresses::live::BATH_HOUSE_ADDRESS), provider.clone()),
            Kovan => build_contract_from_json_dir(json_dir, "BathHouse", parse_address(addresses::kovan::BATH_HOUSE_ADDRESS), provider.clone()),
        }?;
        println!("Flag 3");
        let cc = bath_house_contract.method::<_, (Address, Address)>(
            "getBathPair",
            (
                parse_address(addresses::kovan::ETH_ERC20_ADDRESS),
                parse_address(addresses::kovan::USDC_ERC20_ADDRESS),
            ),
        )?;
        println!("Flag 4");
        let pending_tx = cc.send().await?; // this is erroring out: "custom error: EIP-1559 not activated"
        println!("Flag 4.5");
        let receipt = pending_tx.confirmations(6).await?;
        println!("{:?}", &receipt);
        println!("Flag 5");
        Ok(Self {
            provider: provider,
            bath_house: bath_house_contract,
        })
    }
    /*fn new(provider_url: &str) -> Result<Self> {
        Ok(Self {
            provider: providers::live::http_provider()?,
        })
    }*/
}
