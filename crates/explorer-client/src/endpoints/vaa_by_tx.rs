use std::borrow::Cow;

use reqwest::Url;

use crate::{endpoints::vaa::ExplorerVaaResponse, ApiCall};

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct VaaByTxHashRequest<'a> {
    tx_hash: Cow<'a, str>,
}

impl<'a> From<&'a str> for VaaByTxHashRequest<'a> {
    fn from(tx_hash: &'a str) -> Self {
        Self {
            tx_hash: Cow::Borrowed(tx_hash),
        }
    }
}

impl From<String> for VaaByTxHashRequest<'static> {
    fn from(tx_hash: String) -> Self {
        Self {
            tx_hash: Cow::Owned(tx_hash),
        }
    }
}

impl ApiCall for VaaByTxHashRequest<'_> {
    type Return = ExplorerVaaResponse;

    fn add_endpoint(&self, url: &mut Url) {
        url.set_path("/api/v1/vaas/");
    }

    fn add_query_args(&self, url: &mut reqwest::Url) {
        url.set_query(Some(&format!("txHash={}", self.tx_hash)));
    }
}
