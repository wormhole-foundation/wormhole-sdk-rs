pub mod chain_id;
pub mod deploys;

pub use chain_id::{ChainId, KnownChainIds};

/// Enum representing the VM used by a chain.
pub enum Vm {
    Evm,
    Solana,
    CosmWasm,
}

/// Struct representing the core deployment info for a chain.
pub struct CoreDeployment {
    /// The chain id.
    pub chain_id: ChainId,
    /// The name of the chain.
    pub name: &'static str,
    /// The core contract address on the chain.
    pub core_address: Vec<u8>,
    /// The token bridge contract (if any).
    pub token_bridge_address: Option<Vec<u8>>,
    /// The NFT bridge contract (if any).
    pub nft_bridge_address: Option<Vec<u8>>,
    /// The VM used by the chain.
    pub vm: Vm,
}
