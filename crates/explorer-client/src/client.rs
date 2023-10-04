use crate::{ApiCall, Result};

use reqwest::Url;

/// API client
#[derive(Debug, Clone)]
pub struct Client {
    pub(crate) root: Url,
    pub(crate) client: reqwest::Client,
}

impl std::ops::Deref for Client {
    type Target = reqwest::Client;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

impl Client {
    pub const MAINNET_URL: &'static str = "https://api.wormholescan.io/";
    pub const TESTNET_URL: &'static str = "https://api.testnet.wormholescan.io/";

    /// Instantiate a new API client
    pub fn new(root: Url, client: reqwest::Client) -> Self {
        Self { root, client }
    }

    pub fn mainnet() -> Self {
        Self::new(Self::MAINNET_URL.try_into().unwrap(), Default::default())
    }

    pub fn testnet() -> Self {
        Self::new(Self::TESTNET_URL.try_into().unwrap(), Default::default())
    }

    pub fn send<C: ApiCall>(&self, c: &C) -> impl std::future::Future<Output = Result<C::Return>> {
        c.send(self)
    }
}
