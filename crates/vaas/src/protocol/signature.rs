use alloy_primitives::FixedBytes;

use crate::{Readable, Writeable};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct GuardianSetSig {
    pub guardian_set_index: u8,
    #[cfg_attr(
        feature = "serde",
        serde(with = "crate::support::serde::fixed_bytes_as_array")
    )]
    pub signature: FixedBytes<65>,
}

impl GuardianSetSig {
    pub fn raw_sig(&self) -> [u8; 64] {
        self.signature[0..64].try_into().unwrap()
    }

    pub fn recovery_id(&self) -> u8 {
        self.signature[64]
    }
}

impl Readable for GuardianSetSig {
    fn read<R>(reader: &mut R) -> std::io::Result<Self>
    where
        Self: Sized,
        R: std::io::Read,
    {
        let mut guardian_set_index = [0u8];
        reader.read_exact(&mut guardian_set_index)?;

        Ok(Self {
            guardian_set_index: guardian_set_index[0],
            signature: Readable::read(reader)?,
        })
    }
}

impl Writeable for GuardianSetSig {
    fn write<W>(&self, writer: &mut W) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        writer.write_all(&[self.guardian_set_index])?;
        self.signature.write(writer)?;
        Ok(())
    }
}
