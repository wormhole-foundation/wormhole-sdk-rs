use crate::aliases::U256;
use crate::{Readable, TypePrefixedPayload, Writeable};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SetMessageFee {
    pub chain: u16,
    pub fee: U256,
}

impl TypePrefixedPayload for SetMessageFee {
    const TYPE: Option<u8> = Some(3);
}

impl Readable for SetMessageFee {
    const SIZE: Option<usize> = Some(2 + 32);

    fn read<R>(reader: &mut R) -> std::io::Result<Self>
    where
        Self: Sized,
        R: std::io::Read,
    {
        Ok(Self {
            chain: Readable::read(reader)?,
            fee: Readable::read(reader)?,
        })
    }
}

impl Writeable for SetMessageFee {
    fn write<W>(&self, writer: &mut W) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        self.chain.write(writer)?;
        self.fee.write(writer)
    }

    fn written_size(&self) -> usize {
        <Self as Readable>::SIZE.unwrap()
    }
}
