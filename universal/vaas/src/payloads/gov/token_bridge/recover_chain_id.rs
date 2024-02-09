use crate::{Readable, TypePrefixedPayload, Writeable};
use alloy_primitives::U256;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RecoverChainId {
    pub recovered_chain: u16,
    pub evm_chain_id: U256,
    pub new_chain: u16,
}

impl TypePrefixedPayload for RecoverChainId {
    const TYPE: Option<u8> = Some(3);
}

impl Readable for RecoverChainId {
    const SIZE: Option<usize> = Some(2 + 32 + 2);

    fn read<R>(reader: &mut R) -> std::io::Result<Self>
    where
        Self: Sized,
        R: std::io::Read,
    {
        Ok(Self {
            recovered_chain: Readable::read(reader)?,
            evm_chain_id: Readable::read(reader)?,
            new_chain: Readable::read(reader)?,
        })
    }
}

impl Writeable for RecoverChainId {
    fn write<W>(&self, writer: &mut W) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        self.recovered_chain.write(writer)?;
        self.evm_chain_id.write(writer)?;
        self.new_chain.write(writer)
    }

    fn written_size(&self) -> usize {
        <Self as Readable>::SIZE.unwrap()
    }
}
