use alloy_primitives::{FixedBytes, U256};

use crate::{Readable, Writeable};

use std::io;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransferWithMessage {
    pub(crate) ty: u8,
    pub(crate) amount: U256,
    pub(crate) token_address: FixedBytes<32>,
    pub(crate) token_chain: u16,
    pub(crate) to: FixedBytes<32>,
    pub(crate) to_chain: u16,
    pub(crate) fee: U256,
    pub(crate) payload: Vec<u8>,
}

impl Readable for TransferWithMessage {
    const SIZE: Option<usize> = None;

    fn read<R>(reader: &mut R) -> io::Result<Self>
    where
        Self: Sized,
        R: io::Read,
    {
        Ok(Self {
            ty: Readable::read(reader)?,
            amount: Readable::read(reader)?,
            token_address: Readable::read(reader)?,
            token_chain: Readable::read(reader)?,
            to: Readable::read(reader)?,
            to_chain: Readable::read(reader)?,
            fee: Readable::read(reader)?,
            payload: {
                let mut buf = Vec::new();
                reader.read_to_end(&mut buf)?;
                buf
            },
        })
    }
}

impl Writeable for TransferWithMessage {
    fn written_size(&self) -> usize {
        1 + 32 + 32 + 2 + 32 + 2 + 32 + self.payload.len()
    }

    fn write<W>(&self, writer: &mut W) -> io::Result<()>
    where
        Self: Sized,
        W: io::Write,
    {
        self.ty.write(writer)?;
        self.amount.write(writer)?;
        self.token_address.write(writer)?;
        self.token_chain.write(writer)?;
        self.to.write(writer)?;
        self.to_chain.write(writer)?;
        self.fee.write(writer)?;
        writer.write_all(&self.payload)?;
        Ok(())
    }
}
