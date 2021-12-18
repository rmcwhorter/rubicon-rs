
/*
pub fn new(chain: Chain, api_key: impl Into<String>) -> Result<Self> {
        let (etherscan_api_url, etherscan_url) = match chain {
            Chain::Mainnet => {
                (Url::parse("https://api.etherscan.io/api"), Url::parse("https://etherscan.io"))
            }
            Chain::Ropsten | Chain::Kovan | Chain::Rinkeby | Chain::Goerli => {
                let chain_name = chain.to_string().to_lowercase();

                (
                    Url::parse(&format!("https://api-{}.etherscan.io/api", chain_name)),
                    Url::parse(&format!("https://{}.etherscan.io", chain_name)),
                )
            }
            Chain::Polygon => (
                Url::parse("https://api.polygonscan.com/api"),
                Url::parse("https://polygonscan.com"),
            ),
            Chain::PolygonMumbai => (
                Url::parse("https://api-testnet.polygonscan.com/api"),
                Url::parse("https://mumbai.polygonscan.com"),
            ),
            Chain::Avalanche => {
                (Url::parse("https://api.snowtrace.io/api"), Url::parse("https://snowtrace.io"))
            }
            Chain::AvalancheFuji => (
                Url::parse("https://api-testnet.snowtrace.io/api"),
                Url::parse("https://testnet.snowtrace.io"),
            ),
            chain => return Err(EtherscanError::ChainNotSupported(chain)),
        };

        Ok(Self {
            client: Default::default(),
            api_key: api_key.into(),
            etherscan_api_url: etherscan_api_url.expect("is valid http"),
            etherscan_url: etherscan_url.expect("is valid http"),
        })
*/

use reqwest::{Url};
use ethers::etherscan::Client as ESClient;
use anyhow::Result;
fn new_client(api_key: impl Into<String>, etherscan_base_url: Url, etherscan_api_url: Url) -> Result<ESClient> {
    Ok(ESClient {
        client: Default::default(),
        api_key: api_key.into(),
        etherscan_api_url: etherscan_api_url,
        etherscan_url: etherscan_base_url,
    })
}

mod kovan {
    const KOVAN_ETHERSCAN_BASE_URL: &str = "";
    const KOVAN_ETHERSCAN_API_URL: &str = "";

    
}

mod optimism {

}