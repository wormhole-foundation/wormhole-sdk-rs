use crate::{Readable, Writeable};
use alloy_primitives::FixedBytes;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RegisterChain {
    _gap: FixedBytes<2>, // This should never be encoded with anything.
    pub foreign_chain: u16,
    pub foreign_emitter: FixedBytes<32>,
}

impl Readable for RegisterChain {
    const SIZE: Option<usize> = Some(2 + 2 + 32);

    fn read<R>(reader: &mut R) -> std::io::Result<Self>
    where
        Self: Sized,
        R: std::io::Read,
    {
        let _gap = FixedBytes::<2>::read(reader)?;
        if _gap != FixedBytes::ZERO {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid register chain",
            ));
        }

        Ok(Self {
            _gap,
            foreign_chain: Readable::read(reader)?,
            foreign_emitter: Readable::read(reader)?,
        })
    }
}

impl Writeable for RegisterChain {
    fn write<W>(&self, writer: &mut W) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        self._gap.write(writer)?;
        self.foreign_chain.write(writer)?;
        self.foreign_emitter.write(writer)
    }

    fn written_size(&self) -> usize {
        <Self as Readable>::SIZE.unwrap()
    }
}
