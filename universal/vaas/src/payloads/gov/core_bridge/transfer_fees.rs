use crate::{Readable, TypePrefixedPayload, Writeable};
use alloy_primitives::{FixedBytes, U256};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TransferFees {
    pub chain: u16,
    pub amount: U256,
    pub recipient: FixedBytes<32>,
}

impl TypePrefixedPayload for TransferFees {
    const TYPE: Option<u8> = Some(4);
}

impl Readable for TransferFees {
    const SIZE: Option<usize> = Some(2 + 32 + 32);

    fn read<R>(reader: &mut R) -> std::io::Result<Self>
    where
        Self: Sized,
        R: std::io::Read,
    {
        Ok(Self {
            chain: Readable::read(reader)?,
            amount: Readable::read(reader)?,
            recipient: Readable::read(reader)?,
        })
    }
}

impl Writeable for TransferFees {
    fn write<W>(&self, writer: &mut W) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        self.chain.write(writer)?;
        self.amount.write(writer)?;
        self.recipient.write(writer)
    }

    fn written_size(&self) -> usize {
        <Self as Readable>::SIZE.unwrap()
    }
}
