use alloy_primitives::FixedBytes;
use wormhole_vaas::{Readable, Vaa};

use crate::{ApiCall, Pagination, Result};

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct ExplorerVaaResponse {
    /// The returned data.
    pub data: Vec<ExplorerVaa>,
    /// Pagination information (if any)
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub pagination: Option<Pagination>,
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExplorerVaa {
    pub sequence: u32,
    pub id: String,
    pub version: u8,
    pub emitter_chain: u16,
    pub emitter_addr: FixedBytes<32>,
    pub emitter_native_addr: String,
    #[serde(with = "base64")]
    pub vaa: Vec<u8>,
    pub timestamp: String,
    pub updated_at: String,
    pub indexed_at: String,
    pub tx_hash: Option<String>,
}

impl std::fmt::Debug for ExplorerVaa {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExplorerVaa")
            .field("sequence", &self.sequence)
            .field("id", &self.id)
            .field("version", &self.version)
            .field("emitter_chain", &self.emitter_chain)
            .field("emitter_addr", &self.emitter_addr)
            .field("emitter_native_addr", &self.emitter_native_addr)
            .field("vaa", &hex::encode(&self.vaa))
            .field("timestamp", &self.timestamp)
            .field("updated_at", &self.updated_at)
            .field("indexed_at", &self.indexed_at)
            .field("tx_hash", &self.tx_hash)
            .finish()
    }
}

impl ExplorerVaa {
    pub fn deser_vaa(&self) -> Result<Vaa> {
        Vaa::read(&mut self.vaa.as_slice()).map_err(Into::into)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct VaaRequest {
    pub chain_id: Option<u16>,
    pub emitter: Option<FixedBytes<32>>,
    pub sequence: Option<u64>,
}

impl ApiCall for VaaRequest {
    type Return = ExplorerVaaResponse;

    fn endpoint(&self) -> String {
        let stem = "/api/v1/vaas";

        match (self.chain_id, &self.emitter, self.sequence) {
            (Some(cid), None, _) => format!("{stem}/{cid}"),
            (Some(cid), Some(emitter), None) => format!("{stem}/{cid}/{emitter}"),
            (Some(cid), Some(emitter), Some(seq)) => format!("{stem}/{cid}/{emitter}/{seq}"),
            _ => stem.to_string(),
        }
    }
}

mod base64 {
    use serde::{Deserialize, Serialize};
    use serde::{Deserializer, Serializer};

    use base64::Engine;

    pub fn serialize<S: Serializer>(v: &Vec<u8>, s: S) -> Result<S::Ok, S::Error> {
        let base64 = base64::engine::general_purpose::STANDARD.encode(v);
        String::serialize(&base64, s)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<u8>, D::Error> {
        let base64 = String::deserialize(d)?;
        base64::engine::general_purpose::STANDARD
            .decode(base64.as_bytes())
            .map_err(serde::de::Error::custom)
    }
}
