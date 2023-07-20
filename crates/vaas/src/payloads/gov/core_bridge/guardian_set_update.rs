use crate::{Readable, Writeable};
use alloy_primitives::FixedBytes;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GuardianSetUpdate {
    _gap: FixedBytes<2>, // This should never be encoded with anything.
    pub new_index: u32,
    pub guardians: Vec<FixedBytes<20>>,
}

impl Readable for GuardianSetUpdate {
    const SIZE: Option<usize> = None;

    fn read<R>(reader: &mut R) -> std::io::Result<Self>
    where
        Self: Sized,
        R: std::io::Read,
    {
        let _gap = FixedBytes::<2>::read(reader)?;
        if _gap != FixedBytes::ZERO {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid guardian set update",
            ));
        }
        let new_index = u32::read(reader)?;
        let num_guardians = u8::read(reader)?;
        let guardians: Vec<_> = (0..num_guardians)
            .filter_map(|_| FixedBytes::<20>::read(reader).ok())
            .collect();

        Ok(Self {
            _gap,
            new_index,
            guardians,
        })
    }
}

impl Writeable for GuardianSetUpdate {
    fn write<W>(&self, writer: &mut W) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        self._gap.write(writer)?;
        self.new_index.write(writer)?;

        let guardians = &self.guardians;
        (guardians.len() as u8).write(writer)?;
        for guardian in guardians.iter() {
            guardian.write(writer)?;
        }
        Ok(())
    }

    fn written_size(&self) -> usize {
        2 + 4 + 1 + self.guardians.len() * 20
    }
}
