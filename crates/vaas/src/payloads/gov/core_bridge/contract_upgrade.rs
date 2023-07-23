use crate::{Readable, TypePrefixedPayload, Writeable};
use alloy_primitives::FixedBytes;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContractUpgrade {
    pub chain: u16,
    pub implementation: FixedBytes<32>,
}

impl TypePrefixedPayload for ContractUpgrade {
    const TYPE: Option<u8> = Some(1);
}

impl Readable for ContractUpgrade {
    const SIZE: Option<usize> = Some(2 + 32);

    fn read<R>(reader: &mut R) -> std::io::Result<Self>
    where
        Self: Sized,
        R: std::io::Read,
    {
        Ok(Self {
            chain: Readable::read(reader)?,
            implementation: Readable::read(reader)?,
        })
    }
}

impl Writeable for ContractUpgrade {
    fn write<W>(&self, writer: &mut W) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        self.chain.write(writer)?;
        self.implementation.write(writer)
    }

    fn written_size(&self) -> usize {
        <Self as Readable>::SIZE.unwrap()
    }
}
