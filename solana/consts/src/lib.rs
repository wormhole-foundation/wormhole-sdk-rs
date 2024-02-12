/// Wormhole Chain ID identifying Solana's network. This ID is shared between Solana mainnet and
/// devnet.
pub const SOLANA_CHAIN: u16 = 1;

pub const WRAPPED_MINT_MAX_DECIMALS: u8 = 8;

use solana_program::{pubkey, pubkey::Pubkey};

cfg_if::cfg_if! {
    if #[cfg(feature = "mainnet")] {
        /// Core Bridge program ID on Solana mainnet.
        pub const CORE_BRIDGE_PROGRAM_ID: Pubkey = pubkey!("worm2ZoG2kUd4vFXhvjh93UUH596ayRfgQ2MgjNMTth");
        pub const CORE_BRIDGE_FEE_COLLECTOR: Pubkey = pubkey!("9bFNrXNb2WTx8fMHXCheaZqkLZ3YCCaiqTftHxeintHy");
        pub const CORE_BRIDGE_CONFIG: Pubkey = pubkey!("2yVjuQwpsvdsrywzsJJVs9Ueh4zayyo5DYJbBNc3DDpn");

        /// Token Bridge program ID on Solana mainnet.
        pub const TOKEN_BRIDGE_PROGRAM_ID: Pubkey = pubkey!("wormDTUJ6AWPNvk59vGQbDvGJmqbDTdgWgAqcLBCgUb");
        pub const TOKEN_BRIDGE_EMITTER_AUTHORITY: Pubkey = pubkey!("Gv1KWf8DT1jKv5pKBmGaTmVszqa56Xn8YGx2Pg7i7qAk");
        pub const TOKEN_BRIDGE_CUSTODY_AUTHORITY: Pubkey = pubkey!("GugU1tP7doLeTw9hQP51xRJyS8Da1fWxuiy2rVrnMD2m");
        pub const TOKEN_BRIDGE_MINT_AUTHORITY: Pubkey = pubkey!("BCD75RNBHrJJpW4dXVagL5mPjzRLnVZq4YirJdjEYMV7");
        pub const TOKEN_BRIDGE_TRANSFER_AUTHORITY: Pubkey = pubkey!("7oPa2PHQdZmjSPqvpZN7MQxnC7Dcf3uL4oLqknGLk2S3");

        /// USDC mint address found on Solana mainnet.
        pub const USDC_MINT: Pubkey = pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
    } else if #[cfg(feature = "testnet")] {
        /// Core Bridge program ID on Solana devnet.
        pub const CORE_BRIDGE_PROGRAM_ID: Pubkey = pubkey!("3u8hJUVTA4jH1wYAyUur7FFZVQ8H635K3tSHHF4ssjQ5");
        pub const CORE_BRIDGE_FEE_COLLECTOR: Pubkey = pubkey!("7s3a1ycs16d6SNDumaRtjcoyMaTDZPavzgsmS3uUZYWX");
        pub const CORE_BRIDGE_CONFIG: Pubkey = pubkey!("6bi4JGDoRwUs9TYBuvoA7dUVyikTJDrJsJU1ew6KVLiu");

        /// Token Bridge program ID on Solana devnet.
        pub const TOKEN_BRIDGE_PROGRAM_ID: Pubkey = pubkey!("DZnkkTmCiFWfYTfT41X3Rd1kDgozqzxWaHqsw6W4x2oe");
        pub const TOKEN_BRIDGE_EMITTER_AUTHORITY: Pubkey = pubkey!("4yttKWzRoNYS2HekxDfcZYmfQqnVWpKiJ8eydYRuFRgs");
        pub const TOKEN_BRIDGE_CUSTODY_AUTHORITY: Pubkey = pubkey!("H9pUTqZoRyFdaedRezhykA1aTMq7vbqRHYVhpHZK2QbC");
        pub const TOKEN_BRIDGE_MINT_AUTHORITY: Pubkey = pubkey!("rRsXLHe7sBHdyKU3KY3wbcgWvoT1Ntqudf6e9PKusgb");
        pub const TOKEN_BRIDGE_TRANSFER_AUTHORITY: Pubkey = pubkey!("3VFdJkFuzrcwCwdxhKRETGxrDtUVAipNmYcLvRBDcQeH");

        /// USDC mint address found on Solana devnet.
        pub const USDC_MINT: Pubkey = pubkey!("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU");
    } else if #[cfg(feature = "localnet")] {
        /// Core Bridge program ID on Wormhole's Tilt (dev) network.
        pub const CORE_BRIDGE_PROGRAM_ID: Pubkey = pubkey!("Bridge1p5gheXUvJ6jGWGeCsgPKgnE3YgdGKRVCMY9o");
        pub const CORE_BRIDGE_FEE_COLLECTOR: Pubkey = pubkey!("GXBsgBD3LDn3vkRZF6TfY5RqgajVZ4W5bMAdiAaaUARs");
        pub const CORE_BRIDGE_CONFIG: Pubkey = pubkey!("FKoMTctsC7vJbEqyRiiPskPnuQx2tX1kurmvWByq5uZP");

        /// Token Bridge program ID on Wormhole's Tilt (dev) network.
        pub const TOKEN_BRIDGE_PROGRAM_ID: Pubkey = pubkey!("B6RHG3mfcckmrYN1UhmJzyS1XX3fZKbkeUcpJe9Sy3FE");
        pub const TOKEN_BRIDGE_EMITTER_AUTHORITY: Pubkey = pubkey!("ENG1wQ7CQKH8ibAJ1hSLmJgL9Ucg6DRDbj752ZAfidLA");
        pub const TOKEN_BRIDGE_CUSTODY_AUTHORITY: Pubkey = pubkey!("JCQ1JdJ3vgnvurNAqMvpwaiSwJXaoMFJN53F6sRKejxQ");
        pub const TOKEN_BRIDGE_MINT_AUTHORITY: Pubkey = pubkey!("8P2wAnHr2t4pAVEyJftzz7k6wuCE7aP1VugNwehzCJJY");
        pub const TOKEN_BRIDGE_TRANSFER_AUTHORITY: Pubkey = pubkey!("C1AVBd8PpfHGe1zW42XXVbHsAQf6q5khiRKuGPLbwHkh");

        /// USDC mint address found on Solana devnet.
        ///
        /// NOTE: We expect an integrator to load this account by pulling it from Solana devnet.
        pub const USDC_MINT: Pubkey = pubkey!("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU");
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

    #[test]
    fn token_bridge_id() {
        cfg_if::cfg_if! {
            if #[cfg(feature = "mainnet")] {
                let expected = "wormDTUJ6AWPNvk59vGQbDvGJmqbDTdgWgAqcLBCgUb";
            } else if #[cfg(feature = "testnet")] {
                let expected = "DZnkkTmCiFWfYTfT41X3Rd1kDgozqzxWaHqsw6W4x2oe";
            } else if #[cfg(feature = "localnet")] {
                let expected = "B6RHG3mfcckmrYN1UhmJzyS1XX3fZKbkeUcpJe9Sy3FE";
            }
        }

        assert_eq!(TOKEN_BRIDGE_PROGRAM_ID, Pubkey::from_str(expected).unwrap());
    }

    #[test]
    fn core_bridge_fee_collector() {
        let (expected, _) =
            Pubkey::find_program_address(&[b"fee_collector"], &CORE_BRIDGE_PROGRAM_ID);
        assert_eq!(CORE_BRIDGE_FEE_COLLECTOR, expected);
    }

    #[test]
    fn core_bridge_config() {
        let (expected, _) = Pubkey::find_program_address(&[b"Bridge"], &CORE_BRIDGE_PROGRAM_ID);
        assert_eq!(CORE_BRIDGE_CONFIG, expected);
    }

    #[test]
    fn token_bridge_emitter_authority() {
        let (expected, _) = Pubkey::find_program_address(&[b"emitter"], &TOKEN_BRIDGE_PROGRAM_ID);
        assert_eq!(TOKEN_BRIDGE_EMITTER_AUTHORITY, expected);
    }

    #[test]
    fn token_bridge_custody_authority() {
        let (expected, _) =
            Pubkey::find_program_address(&[b"custody_signer"], &TOKEN_BRIDGE_PROGRAM_ID);
        assert_eq!(TOKEN_BRIDGE_CUSTODY_AUTHORITY, expected);
    }

    #[test]
    fn token_bridge_mint_authority() {
        let (expected, _) =
            Pubkey::find_program_address(&[b"mint_signer"], &TOKEN_BRIDGE_PROGRAM_ID);
        assert_eq!(TOKEN_BRIDGE_MINT_AUTHORITY, expected);
    }

    #[test]
    fn token_bridge_transfer_authority() {
        let (expected, _) =
            Pubkey::find_program_address(&[b"authority_signer"], &TOKEN_BRIDGE_PROGRAM_ID);
        assert_eq!(TOKEN_BRIDGE_TRANSFER_AUTHORITY, expected);
    }
}
