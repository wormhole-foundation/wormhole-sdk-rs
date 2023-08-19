use std::collections::HashMap;

use alloy_primitives::{FixedBytes, U256};

use crate::ApiCall;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExplorerTxResponse {
    pub transactions: Vec<ExplorerTx>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExplorerTx {
    pub id: String,
    pub emitter_chain: u16,
    pub emitter_address: FixedBytes<32>,

    pub tx_hash: Option<String>,

    pub emitter_native_addr: Option<String>,
    pub global_tx: Option<GlobalTx>,
    pub symbol: Option<String>,
    pub timestamp: Option<String>,
    pub token_amount: Option<String>,
    pub usd_amount: Option<String>,

    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub payload: HashMap<String, serde_json::Value>,

    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub standardized_properties: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GlobalTx {
    pub destination_tx: Option<DestinationTx>,
    pub id: String,
    pub origin_tx: OriginTx,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinationTx {
    pub block_number: U256,
    pub chain_id: u16,
    pub from: String,
    pub method: String,
    pub status: String,
    pub timestamp: String,
    pub to: String,
    pub tx_hash: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OriginTx {
    pub from: String,
    pub status: String,
    pub tx_hash: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AllTxnsRequest;

impl ApiCall for AllTxnsRequest {
    type Return = ExplorerTxResponse;

    fn endpoint(&self) -> String {
        "/api/v1/transactions/".to_string()
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SingleTxRequest {
    pub chain_id: u16,
    pub emitter: FixedBytes<32>,
    pub sequence: u64,
}

impl ApiCall for SingleTxRequest {
    type Return = ExplorerTx;

    fn endpoint(&self) -> String {
        format!(
            "/api/v1/transactions/{}/{}/{}",
            self.chain_id, self.emitter, self.sequence
        )
    }
}
