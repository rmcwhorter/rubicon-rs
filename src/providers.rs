pub mod live {
    use crate::addresses;
    use anyhow::Result;
    use ethers::providers::{Http, Provider};

    pub fn http_provider() -> Result<Provider<Http>> {
        Ok(Provider::<Http>::try_from(
            addresses::live::LIVE_OPTIMISM_PROVIDER_URL_HTTPS,
        )?)
    }
}
pub mod kovan {
    use crate::addresses;
    use anyhow::Result;
    use ethers::providers::{Http, Provider};

    pub fn http_provider() -> Result<Provider<Http>> {
        Ok(Provider::<Http>::try_from(
            addresses::kovan::KOVAN_OPTIMISM_PROVIDER_URL_HTTPS,
        )?)
    }
}
