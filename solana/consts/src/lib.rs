/// Wormhole Chain ID identifying Solana's network. This ID is shared between Solana mainnet and
/// devnet.
pub const SOLANA_CHAIN: u16 = 1;

use solana_program::{pubkey, pubkey::Pubkey};

cfg_if::cfg_if! {
    if #[cfg(feature = "mainnet")] {
        /// The Core Bridge program ID on Solana mainnet.
        pub const CORE_BRIDGE_PROGRAM_ID: Pubkey = pubkey!("worm2ZoG2kUd4vFXhvjh93UUH596ayRfgQ2MgjNMTth");
    } else if #[cfg(feature = "testnet")] {
        /// The Core Bridge program ID on Solana devnet.
        pub const CORE_BRIDGE_PROGRAM_ID: Pubkey = pubkey!("3u8hJUVTA4jH1wYAyUur7FFZVQ8H635K3tSHHF4ssjQ5");
    } else if #[cfg(feature = "localnet")] {
        /// The Core Bridge program ID on Wormhole's Tilt (dev) network.
        pub const CORE_BRIDGE_PROGRAM_ID: Pubkey = pubkey!("Bridge1p5gheXUvJ6jGWGeCsgPKgnE3YgdGKRVCMY9o");
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn core_bridge_id() {
        cfg_if::cfg_if! {
            if #[cfg(feature = "mainnet")] {
                let expected = "worm2ZoG2kUd4vFXhvjh93UUH596ayRfgQ2MgjNMTth";
            } else if #[cfg(feature = "testnet")] {
                let expected = "3u8hJUVTA4jH1wYAyUur7FFZVQ8H635K3tSHHF4ssjQ5";
            } else if #[cfg(feature = "localnet")] {
                let expected = "Bridge1p5gheXUvJ6jGWGeCsgPKgnE3YgdGKRVCMY9o";
            }
        }

        assert_eq!(CORE_BRIDGE_PROGRAM_ID, Pubkey::from_str(expected).unwrap());
    }
}
