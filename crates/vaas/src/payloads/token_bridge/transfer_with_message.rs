use alloy_primitives::{FixedBytes, U256};

use crate::{Readable, TypePrefixedPayload, Writeable};

use std::io;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransferWithMessage {
    norm_amount: U256,
    token_address: FixedBytes<32>,
    token_chain: u16,
    redeemer: FixedBytes<32>,
    redeemer_chain: u16,
    sender: FixedBytes<32>,
    payload: Vec<u8>,
}

impl TypePrefixedPayload for TransferWithMessage {
    const TYPE: Option<u8> = Some(3);
}

impl Readable for TransferWithMessage {
    const SIZE: Option<usize> = None;

    fn read<R>(reader: &mut R) -> io::Result<Self>
    where
        Self: Sized,
        R: io::Read,
    {
        Ok(Self {
            norm_amount: Readable::read(reader)?,
            token_address: Readable::read(reader)?,
            token_chain: Readable::read(reader)?,
            redeemer: Readable::read(reader)?,
            redeemer_chain: Readable::read(reader)?,
            sender: Readable::read(reader)?,
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
        32 + 32 + 2 + 32 + 2 + 32 + self.payload.len()
    }

    fn write<W>(&self, writer: &mut W) -> io::Result<()>
    where
        Self: Sized,
        W: io::Write,
    {
        self.norm_amount.write(writer)?;
        self.token_address.write(writer)?;
        self.token_chain.write(writer)?;
        self.redeemer.write(writer)?;
        self.redeemer_chain.write(writer)?;
        self.sender.write(writer)?;
        writer.write_all(&self.payload)?;
        Ok(())
    }
}
