use alloy_primitives::FixedBytes;

use crate::{Readable, Writeable};

use std::io;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Attestation {
    pub ty: u8,
    pub token_address: FixedBytes<32>,
    pub token_chain: u16,
    pub decimals: u8,

    pub symbol: FixedBytes<32>,
    pub name: FixedBytes<32>,
}

impl Attestation {
    pub fn symbol_string(&self) -> String {
        let bytes = self
            .symbol
            .rsplit(|b| *b == 0)
            .next()
            .unwrap_or(self.symbol.as_slice());

        String::from_utf8_lossy(bytes).into_owned()
    }

    pub fn name_string(&self) -> String {
        let bytes = self
            .name
            .rsplit(|b| *b == 0)
            .next()
            .unwrap_or(self.name.as_slice());

        String::from_utf8_lossy(bytes).into_owned()
    }
}

impl Readable for Attestation {
    fn read<R>(reader: &mut R) -> io::Result<Self>
    where
        Self: Sized,
        R: io::Read,
    {
        Ok(Self {
            ty: Readable::read(reader)?,
            token_address: Readable::read(reader)?,
            token_chain: Readable::read(reader)?,
            decimals: Readable::read(reader)?,
            symbol: Readable::read(reader)?,
            name: Readable::read(reader)?,
        })
    }
}

impl Writeable for Attestation {
    fn write<W>(&self, writer: &mut W) -> io::Result<()>
    where
        Self: Sized,
        W: io::Write,
    {
        self.ty.write(writer)?;
        self.token_address.write(writer)?;
        self.token_chain.write(writer)?;
        self.decimals.write(writer)?;
        self.symbol.write(writer)?;
        self.name.write(writer)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use alloy_primitives::{FixedBytes, U64};
    use hex_literal::hex;

    use crate::{payloads::portal::Attestation, Readable, Vaa, Writeable};

    #[test]
    fn parse_token_bridge_attestation() {
        let vaa = hex!("010000000001006cd3cdd701bbd878eb403f6505b5b797544eb9c486dadf79f0c445e9b8fa5cd474de1683e3a80f7e22dbfacd53b0ddc7b040ff6f974aafe7a6571c9355b8129b00000000007ce2ea3f000195f83a27e90c622a98c037353f271fd8f5f57b4dc18ebf5ff75a934724bd0491a43a1c0020f88a3e2002000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc200021200000000000000000000000000000000000000000000000000000000574554480000000000000000000000000000000000000057726170706564206574686572");
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

        let attestation = vaa.body.read_payload::<Attestation>().unwrap();
        assert_eq!(attestation.to_vec(), vaa.body.payload_bytes);

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
                "4bb52b9a44ff6062ba5db1c47afc40c186f7485c8972b1c6261eb070ce0b1c6e"
            ))
        );
    }
}
