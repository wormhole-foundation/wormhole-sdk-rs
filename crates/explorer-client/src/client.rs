use crate::{
    endpoints::{
        vaa::{ExplorerVaa, ExplorerVaaResponse},
        vaa_by_tx::VaaByTxHashRequest,
    },
    ApiCall, Result, VaaRequest,
};

use alloy_primitives::FixedBytes;
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

    pub async fn send<C: ApiCall>(&self, c: &C) -> Result<C::Return> {
        c.send(self).await
    }

    pub async fn fetch_vaas(
        &self,
        chain_id: Option<u16>,
        emitter: Option<FixedBytes<32>>,
        sequence: Option<u64>,
    ) -> Result<Vec<ExplorerVaa>> {
        self.send(&VaaRequest {
            chain_id,
            emitter,
            sequence,
        })
        .await
        .map(|resp| resp.data)
    }

    pub async fn fetch_vaa(
        &self,
        chain_id: u16,
        emitter: FixedBytes<32>,
        sequence: u64,
    ) -> Result<Option<ExplorerVaa>> {
        self.send(&VaaRequest {
            chain_id: Some(chain_id),
            emitter: Some(emitter),
            sequence: Some(sequence),
        })
        .await
        .map(|mut resp: ExplorerVaaResponse| resp.data.drain(..).next())
    }

    pub async fn fetch_vaas_by_tx(&self, tx_hash: &str) -> Result<Vec<ExplorerVaa>> {
        self.send(&VaaByTxHashRequest::from(tx_hash))
            .await
            .map(|resp| resp.data)
    }
}
