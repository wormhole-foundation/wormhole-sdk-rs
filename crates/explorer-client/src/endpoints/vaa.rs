use alloy_primitives::FixedBytes;
use wormhole_vaas::{Readable, Vaa};

use crate::{ApiCall, Pagination, Result};

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct ExplorerVaaResponse {
    #[serde(deserialize_with = "crate::common::one_or_many::deserialize")]
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
    #[serde(with = "crate::common::base64")]
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

#[cfg(test)]
mod test {
    #[test]
    fn deserialize_one() {
        let json = r#"
        {"data":{"sequence":276319,"id":"4/000000000000000000000000b6f6d86a8f9879a9c87f643768d9efc38c1da6e7/276319","version":1,"emitterChain":4,"emitterAddr":"000000000000000000000000b6f6d86a8f9879a9c87f643768d9efc38c1da6e7","emitterNativeAddr":"0xb6f6d86a8f9879a9c87f643768d9efc38c1da6e7","guardianSetIndex":3,"vaa":"AQAAAAMNADNZINhn9wy5s5YAu/1kY0fyWT53DVEXg821bWnAF1PDHJMugUgOqwHJWWCCzab7Ko/k5Nd1gdHPISJJcQ9W7TAAAuINA/6ie9TyHDqfn7d31AWbG0/oXEUeFIuuL6UmnzohQNwU1yS/vYP5k0DyBX1ivAIOeQPcm2WK8PYE84ATOwwBA6ehL0tZ5qPefsPsF7BDjz+ArPY/EVveh262VQK03xZTCurJjlPFsU2HkD6lIacw+K23v93MZXya5oHfdd0HIUIBBABB6r2LV5Ke7yRDp8QD8TanEVEBd9V2AQvNWa8Z6kSaD03VeGUf0quTFaJFsZR3GYJQk7H3hcNOlHb2+FdsuWoABnnBa85DrkswM7q7cveqvKPB8nMBYTtVoTWNLB9GT1KzT67syadZIFdyMtYTPPi9j/aMoVXY1bJx9NUtE/AI15sBCeiqDTXpumVeBEF5zQwz26WzSnKZjPtIjJRMuuRlOVGJMF0T54TWtWlQW/qz9h8kwmd/36vzqFYDxp++ESBgsWYACwin+aSTzgMggaBLx/NqjRTJ/++PIgXOgctTTr+95WUTQVs2TEb/eZ13TXjQ5tM3sIcU93NA0IBAf3CmO2Yi3N8BDGC+7WMzGjnxsVY59gVAbA3EKnQRmVJO592mevX8L4+tOcU2XNHK2LqY71diEZ1t8wn48EaC1ByHndjYAKdhoYgBDWC0fWEFWkg7haEOSTcHrV/fxYufw2ozG9E/I2+ULXEgTO9iA8oWM8jloyvwIQ/QjoYyxz+kwwLZdVztGfqf5nIADsiMJw8Sl8y0FRQMOe5TQlQ1YfOuG464AmOEqWLQiJw2OHgBxCFqJZ50JWIj9L/Ywg5cQGqo5jsF0QlGHak2sKABEPWhC4N6dJiQujpFF0I1vapgsM4hGrSr8kwFYhgs43ONE+F50p6onhO7HxyHgg/Y7E37R49DEbDyE52ksuYYdoMBETseKzat0CZ5wkhzvJRipZyYpZBvvl0ZRYzK6WJKjCliRnB9ERlNUyix1L0mFba/1oJMtcsRckRtjWAM5/uIB9wAEhWWZjjLf600c3CLxqitntc1FXezCm+G9DFe4vRL4fs7DyLhbLjDrmA58eTUuAhdf4Jbzdd74XxnpRGwqMIUu/8BZRNbYpu9AAAABAAAAAAAAAAAAAAAALb22GqPmHmpyH9kN2jZ78OMHabnAAAAAAAEN18PAQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA7AnZRlwDhZ9DbAln7g7yjOJR85C/iw0uAMoXH6Zsmh0vYO6wKgAAaLohJsvKw49z0OqKjUh78eQDBaslYf2QHNwOP+7BfSFAAEAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA==","timestamp":"2023-09-26T22:29:54Z","updatedAt":"2023-09-26T22:30:51.888Z","indexedAt":"2023-09-26T22:30:51.888Z","txHash":"6e55b586b6cf34115bf9b8e6c4045a54e48b9feea919fd846ebbb3f7c122d5bb"},"pagination":{"next":""}}"#;

        let res: super::ExplorerVaaResponse = serde_json::from_str(json).unwrap();
        assert_eq!(res.data.len(), 1);
    }
}
