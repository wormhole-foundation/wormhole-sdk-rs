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
    /// Instantiate a new API client
    pub fn new(root: Url, client: reqwest::Client) -> Self {
        Self { root, client }
    }

    pub fn mainnet() -> Self {
        Self::new(
            "https://api.wormscan.io/".try_into().unwrap(),
            Default::default(),
        )
    }

    pub fn send<C: ApiCall>(&self, c: &C) -> impl std::future::Future<Output = Result<C::Return>> {
        c.send(self)
    }
}
