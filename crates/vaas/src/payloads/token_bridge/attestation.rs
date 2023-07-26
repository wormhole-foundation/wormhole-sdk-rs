use alloy_primitives::FixedBytes;

use crate::{Readable, TypePrefixedPayload, Writeable};

use std::io;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Attestation {
    pub token_address: FixedBytes<32>,
    pub token_chain: u16,
    pub decimals: u8,

    pub symbol: FixedBytes<32>,
    pub name: FixedBytes<32>,
}

impl Attestation {
    pub fn symbol_string(&self) -> String {
        fixed32_to_string(self.symbol)
    }

    pub fn name_string(&self) -> String {
        fixed32_to_string(self.name)
    }
}

impl TypePrefixedPayload for Attestation {
    const TYPE: Option<u8> = Some(2);
}

impl Readable for Attestation {
    const SIZE: Option<usize> = Some(32 + 2 + 1 + 32 + 32);

    fn read<R>(reader: &mut R) -> io::Result<Self>
    where
        Self: Sized,
        R: io::Read,
    {
        Ok(Self {
            token_address: Readable::read(reader)?,
            token_chain: Readable::read(reader)?,
            decimals: Readable::read(reader)?,
            symbol: Readable::read(reader)?,
            name: Readable::read(reader)?,
        })
    }
}

impl Writeable for Attestation {
    fn written_size(&self) -> usize {
        <Self as Readable>::SIZE.unwrap()
    }

    fn write<W>(&self, writer: &mut W) -> io::Result<()>
    where
        Self: Sized,
        W: io::Write,
    {
        self.token_address.write(writer)?;
        self.token_chain.write(writer)?;
        self.decimals.write(writer)?;
        self.symbol.write(writer)?;
        self.name.write(writer)?;
        Ok(())
    }
}

fn fixed32_to_string(fixed: FixedBytes<32>) -> String {
    let idx = fixed
        .iter()
        .rposition(|x| *x != 0)
        .map(|i| i + 1)
        .unwrap_or_default();
    String::from_utf8_lossy(&fixed[..idx]).into_owned()
}

#[cfg(test)]
mod test {
    use alloy_primitives::{FixedBytes, U64};
    use hex_literal::hex;

    use crate::{
        payloads::token_bridge::{attestation::fixed32_to_string, TokenBridgeMessage},
        Readable, Vaa, Writeable,
    };

    #[test]
    fn unicode_truncation_empty() {
        let converted = FixedBytes::<32>::ZERO;
        let recovered = fixed32_to_string(converted);
        assert_eq!(recovered, String::new());
    }

    #[test]
    fn unicode_truncation_small() {
        let input = String::from("🔥");
        let converted = {
            let mut out = [0; 32];
            out[..input.len()].copy_from_slice(input.as_bytes());
            FixedBytes::<32>::from(out)
        };
        let recovered = fixed32_to_string(converted);
        assert_eq!(recovered, String::from("🔥"));
    }

    #[test]
    fn unicode_truncation_exact() {
        let input = String::from("🔥🔥🔥🔥🔥🔥🔥🔥");
        let converted = {
            let mut out = [0; 32];
            out.copy_from_slice(input.as_bytes());
            FixedBytes::<32>::from(out)
        };
        let recovered = fixed32_to_string(converted);
        assert_eq!(recovered, String::from("🔥🔥🔥🔥🔥🔥🔥🔥"));
    }

    #[test]
    fn unicode_truncation_large() {
        let input = String::from("🔥🔥🔥🔥🔥🔥🔥🔥🔥");
        let converted = {
            let mut out = [0; 32];
            out.copy_from_slice(&input.as_bytes()[..32]);
            FixedBytes::<32>::from(out)
        };
        let recovered = fixed32_to_string(converted);
        assert_eq!(recovered, String::from("🔥🔥🔥🔥🔥🔥🔥🔥"));
    }

    #[test]
    fn unicode_truncation_partial_overflow() {
        let input = String::from("0000000000000000000000000000000🔥");
        let converted = {
            let mut out = [0; 32];
            out.copy_from_slice(&input.as_bytes()[..32]);
            FixedBytes::<32>::from(out)
        };
        let recovered = fixed32_to_string(converted);
        assert_eq!(recovered, String::from("0000000000000000000000000000000�"));
    }

    #[test]
    fn parse_token_bridge_attestation() {
        let vaa = hex!("01000000000100ff7edcd3facb7dd6e06e0bd3e178cfddd775208f3e09f0b68bba981b812258716e6e5cd42c0ba413586df1e4066e29a1a41f9a49ae05a58f5fa93590d165abf100000000007ce2ea3f000195f83a27e90c622a98c037353f271fd8f5f57b4dc18ebf5ff75a934724bd0491a43a1c0020f88a3e2002000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc200021257455448000000000000000000000000000000000000000000000000000000005772617070656420657468657200000000000000000000000000000000000000");
        let vaa = Vaa::read(&mut vaa.as_slice()).unwrap();
        assert_eq!(vaa.header.version, 1);
        assert_eq!(vaa.header.guardian_set_index, 0);
        assert_eq!(vaa.header.signatures.len(), 1);

        assert_eq!(vaa.body.timestamp, 0);
        assert_eq!(vaa.body.nonce, 2095245887);
        assert_eq!(vaa.body.emitter_chain, 1);
        assert_eq!(
            vaa.body.emitter_address,
            hex!("95f83a27e90c622a98c037353f271fd8f5f57b4dc18ebf5ff75a934724bd0491")
        );
        assert_eq!(vaa.body.sequence, U64::from(11833801757748136510u64));
        assert_eq!(vaa.body.consistency_level, 32);

        let msg = vaa.body.read_payload::<TokenBridgeMessage>().unwrap();
        assert_eq!(msg.to_vec(), vaa.body.payload_bytes().unwrap());

        if let TokenBridgeMessage::Attestation(attestation) = msg {
            assert_eq!(
                attestation.token_address,
                hex!("000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2")
            );
            assert_eq!(attestation.token_chain, 2);
            assert_eq!(attestation.decimals, 18);

            assert_eq!(attestation.symbol_string(), "WETH");
            assert_eq!(attestation.name_string(), "Wrapped ether");

            assert_eq!(
                vaa.body.double_digest(),
                FixedBytes(hex!(
                    "6793c77cc9283df50ab5f2cdd688637d6ba935d4e6baabf46e07b83c55655461"
                ))
            );
        } else {
            panic!("Wrong message type");
        }
    }
}
